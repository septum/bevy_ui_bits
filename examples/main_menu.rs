use bevy::{color::palettes, input::keyboard::KeyboardInput, prelude::*};
use bevy_ui_bits::*;

const PLAY_BUTTON_ID: usize = 1;
const QUIT_BUTTON_ID: usize = 2;

#[derive(Resource)]
struct SelectedButton {
    pub id: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Main Menu Example".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(SelectedButton { id: PLAY_BUTTON_ID })
        .add_systems(Startup, (spawn_camera, spawn_main_menu))
        .add_systems(
            Update,
            (handle_keyboard_input, handle_mouse_input, update_buttons).chain(),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn spawn_main_menu(mut commands: Commands) {
    let font = &Handle::default();

    let root = Root::new();
    let container = Container::height(Val::Px(550.0)).justify_between();
    let top = Container::new();
    let bottom = Container::height(Val::Px(200.0)).justify_between();
    let actions = Container::new();
    let footer = Container::new();

    let title = EmbossedText::extra_large("My\nGame", font).color(palettes::css::GOLDENROD.into());
    let play = UiButton::rectangle()
        .id(PLAY_BUTTON_ID)
        .background_color(palettes::css::GOLDENROD.into());
    let play_text = EmbossedText::medium("Play", font);
    let quit = UiButton::rectangle().id(QUIT_BUTTON_ID);
    let quit_text = EmbossedText::medium("Quit", font);
    let instructions = SimpleText::small(
        "Use mouse or the arrow keys to interact with the buttons",
        font,
    );

    commands.spawn((
        root,
        children![(
            container,
            children![
                (top, children![title]),
                (
                    bottom,
                    children![
                        (
                            actions,
                            children![(play, children![play_text]), (quit, children![quit_text])]
                        ),
                        (footer, children![instructions])
                    ]
                )
            ]
        )],
    ));
}

fn handle_keyboard_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut selected_button: ResMut<SelectedButton>,
    query: Query<&UiButtonData>,
) {
    for event in keyboard_input_events.read() {
        if event.state.is_pressed() {
            match event.key_code {
                KeyCode::ArrowUp | KeyCode::ArrowDown => {
                    for button_data in &query {
                        if button_data.id == selected_button.id {
                            selected_button.id = if button_data.id == PLAY_BUTTON_ID {
                                QUIT_BUTTON_ID
                            } else {
                                PLAY_BUTTON_ID
                            };

                            break;
                        }
                    }
                }
                _ => break,
            }
        }
    }
}

fn handle_mouse_input(
    mut selected_button: ResMut<SelectedButton>,
    query: Query<(&UiButtonData, &Interaction), (Changed<Interaction>, With<UiButtonData>)>,
) {
    for (button_data, interaction) in &query {
        match *interaction {
            Interaction::Hovered => selected_button.id = button_data.id,
            _ => {}
        }
    }
}

fn update_buttons(
    selected_button: Res<SelectedButton>,
    mut query: Query<(&UiButtonData, &mut BackgroundColor)>,
) {
    for (button_data, mut background_color) in &mut query {
        if button_data.id == selected_button.id {
            *background_color = palettes::css::GOLDENROD.into();
        } else {
            *background_color = Color::NONE.into();
        }
    }
}
