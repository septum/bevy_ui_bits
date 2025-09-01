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
        .add_systems(Startup, (spawn_camera, spawn_main_menu).chain())
        .add_systems(Update, handle_mouse_input)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn spawn_main_menu(mut commands: Commands) {
    let font = &Handle::default();

    let root = Root::new();

    let container = Container::height(Val::Px(400.0)).justify_between();
    let top = Container::height(Val::Auto);
    let bottom = Container::height(Val::Px(200.0)).justify_between();
    let actions = Container::height(Val::Auto);
    let footer = Container::height(Val::Auto);

    let title = EmbossedText::extra_large("Game Over", font).color(palettes::css::TEAL.into());
    let despawn = UiButton::rectangle()
        .id(DESPAWN_BUTTON_ID)
        .background_color(palettes::css::TEAL.into());
    let despawn_text = EmbossedText::medium("Despawn UI", font);
    let instructions = SimpleText::small("Use the mouse to interact with the buttons", font);

    commands.spawn((
        root,
        children![(
            container,
            children![
                (top, children![title]),
                (
                    bottom,
                    children![
                        (actions, children![(despawn, children![despawn_text])]),
                        (footer, children![instructions])
                    ]
                )
            ]
        )],
    ));
}

fn handle_mouse_input(
    mut commands: Commands,
    mut query: Query<
        (&UiButtonData, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
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
                    let root_entity = root_query.single().expect("An entity should be present");
                    commands.entity(root_entity).despawn();
                }
            }
        }
    }
}
