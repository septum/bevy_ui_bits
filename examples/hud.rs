use bevy::{color::palettes, input::keyboard::KeyboardInput, prelude::*};
use bevy_ui_bits::*;

const JUMPS_TEXT_ID: usize = 1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "HUD Example".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (spawn_camera, spawn_hud).chain())
        .add_systems(Update, handle_input)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn spawn_hud(mut commands: Commands) {
    let font = &Handle::default();

    let root = Root::new()
        .padding(UiRect::all(Val::Px(20.0)))
        .justify_between();

    let top = Container::width(Val::Percent(100.0))
        .row()
        .justify_between();
    let bottom = Container::new();

    let level = EmbossedText::medium("Level 1", font).color(palettes::css::GOLD.into());
    let jumps = DynamicTextBuilder::medium("Jumps: ", font)
        .initial_dynamic_text("0")
        .id(JUMPS_TEXT_ID);
    let instructions = SimpleText::small("Press [SPACE] to jump", font);

    commands.spawn((
        root,
        children![
            (top, children![level, jumps.build()]),
            (bottom, children![instructions])
        ],
    ));
}

fn handle_input(
    mut writer: TextUiWriter,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut texts: Query<(Entity, &DynamicTextData)>,
) {
    for event in keyboard_input_events.read() {
        if event.state.is_pressed() {
            match event.key_code {
                KeyCode::Space => {
                    for (entity, data) in texts.iter_mut() {
                        if matches!(data.id, JUMPS_TEXT_ID) {
                            if let Some(mut text) = writer.get_text(entity, 1) {
                                *text =
                                    format!("{}", text.as_str().parse::<usize>().unwrap_or(0) + 1);
                            }
                        }
                    }
                }
                _ => break,
            }
        }
    }
}
