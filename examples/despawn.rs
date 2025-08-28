use bevy::{color::palettes, prelude::*};
use bevy_ui_bits::*;

const DESPAWN_BUTTON_ID: usize = 1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Despawn Example".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (spawn_camera, spawn_main_menu))
        .add_systems(Update, (handle_mouse_input).chain())
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn spawn_main_menu(mut commands: Commands) {
    let font = &Handle::default();

    let root = Root::congregated();

    let mut main_container = Container::height(400.0);
    let top_wrapper = Container::auto();
    let mut bottom_wrapper = Container::height(200.0);
    let actions_wrapper = Container::auto();
    let footer_wrapper = Container::auto();

    let mut title = EmbossedText::extra_large("Game Over", font);
    let instructions = SimpleText::small("Use the mouse to interact with the buttons", font);

    let mut despawn = UiButton::new("Despawn UI", font);

    title.color(palettes::css::TEAL.into());

    main_container.justify_between();
    bottom_wrapper.justify_between();

    despawn
        .id(DESPAWN_BUTTON_ID)
        .selected_color(palettes::css::TEAL.into());

    root.spawn(&mut commands, |parent| {
        main_container.spawn(parent, |parent| {
            top_wrapper.spawn(parent, |parent| {
                title.spawn(parent);
            });
            bottom_wrapper.spawn(parent, |parent| {
                actions_wrapper.spawn(parent, |parent| {
                    despawn.spawn(parent);
                });
                footer_wrapper.spawn(parent, |parent| {
                    instructions.spawn(parent);
                });
            });
        });
    });
}

fn handle_mouse_input(
    mut commands: Commands,
    mut query: Query<
        (&UiButtonData, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<UiButtonData>),
    >,
    root_query: Query<Entity, With<RootMarker>>,
) {
    for (button_data, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = palettes::css::TEAL.into();
            }
            Interaction::Hovered => {
                *background_color = palettes::css::AQUA.into();
            }
            Interaction::Pressed => {
                if button_data.id == DESPAWN_BUTTON_ID {
                    let root_entity = root_query.single();
                    commands.entity(root_entity).despawn_recursive();
                }
            }
        }
    }
}
