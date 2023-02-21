use bevy::{prelude::*, window::close_on_esc};
use bevy_ui_bits::*;

const JUMPS_TEXT_ID: usize = 1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "HUD Example".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_hud)
        .add_system(handle_input)
        .add_system(close_on_esc)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = &asset_server.load("fonts/FiraMono-Medium.ttf");

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

fn handle_input(keys: Res<Input<KeyCode>>, mut texts: Query<(&mut Text, &DynamicTextData)>) {
    if keys.just_pressed(KeyCode::Space) {
        for (mut text, data) in texts.iter_mut() {
            if matches!(data.id, JUMPS_TEXT_ID) {
                text.sections[1].value =
                    format!("{}", text.sections[1].value.parse::<usize>().unwrap() + 1);
            }
        }
    }
}
