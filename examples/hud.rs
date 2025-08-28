use bevy::{input::keyboard::KeyboardInput, prelude::*, window::close_on_esc};
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
        .add_systems(Startup, (spawn_camera, spawn_hud))
        .add_systems(Update, (handle_input, close_on_esc))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_hud(mut commands: Commands) {
    let font = &Handle::default();

    let root = Root::dispersed();
    let mut top_container = Container::auto_height();
    let bottom_container = Container::auto_height();

    let mut level = EmbossedText::medium("Level 1", font);
    let mut jumps = DynamicText::medium("Jumps: ", font);
    let instructions = SimpleText::small("Press [SPACE] to jump", font);

    top_container.row().justify_between();

    level.color(Color::GOLD).background_color(Color::ORANGE_RED);
    jumps.id(JUMPS_TEXT_ID).dynamic_text_value("0");

    root.spawn(&mut commands, |parent| {
        top_container.spawn(parent, |parent| {
            level.spawn(parent);
            jumps.spawn(parent);
        });
        bottom_container.spawn(parent, |parent| {
            instructions.spawn(parent);
        });
    });
}

fn handle_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut texts: Query<(&mut Text, &DynamicTextData)>,
) {
    for event in keyboard_input_events.read() {
        if event.state.is_pressed() {
            match event.key_code {
                KeyCode::Space => {
                    for (mut text, data) in texts.iter_mut() {
                        if matches!(data.id, JUMPS_TEXT_ID) {
                            text.sections[1].value =
                                format!("{}", text.sections[1].value.parse::<usize>().unwrap() + 1);
                        }
                    }
                }
                _ => break,
            }
        }
    }
}
