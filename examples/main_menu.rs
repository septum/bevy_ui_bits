use bevy::{prelude::*, window::close_on_esc};
use bevy_ui_bits::*;

const PLAY_BUTTON_ID: usize = 1;
const QUIT_BUTTON_ID: usize = 2;

#[derive(Resource)]
struct SelectedButton(pub usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Main Menu Example".to_string(),
                ..default()
            },
            ..default()
        }))
        .insert_resource(SelectedButton(PLAY_BUTTON_ID))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_main_menu)
        .add_system(close_on_esc)
        .add_system(handle_keyboard_input)
        .add_system(handle_mouse_input)
        .add_system(update_buttons)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = &asset_server.load("fonts/FiraMono-Medium.ttf");

    let root = Root::congregated();

    let mut main_container = Container::height(500.0);
    let top_wrapper = Container::auto();
    let mut bottom_wrapper = Container::height(200.0);
    let actions_wrapper = Container::auto();
    let footer_wrapper = Container::auto();

    let mut title = EmbossedText::extra_large("My\nGame", font);
    let instructions = SimpleText::small(
        "Use mouse or the arrow keys to interact with the buttons",
        font,
    );

    let mut play = UiButton::new("Play", font);
    let mut quit = UiButton::new("Quit", font);

    title.color(Color::GOLD);

    main_container.justify_between();
    bottom_wrapper.justify_between();

    play.id(PLAY_BUTTON_ID).selected_color(Color::GOLD);

    quit.id(QUIT_BUTTON_ID);

    root.spawn(&mut commands, |parent| {
        main_container.spawn(parent, |parent| {
            top_wrapper.spawn(parent, |parent| {
                title.spawn(parent);
            });
            bottom_wrapper.spawn(parent, |parent| {
                actions_wrapper.spawn(parent, |parent| {
                    play.spawn(parent);
                    quit.spawn(parent);
                });
                footer_wrapper.spawn(parent, |parent| {
                    instructions.spawn(parent);
                });
            });
        });
    });
}

fn handle_keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut selected_button: ResMut<SelectedButton>,
    query: Query<&UiButtonData>,
) {
    if keys.just_pressed(KeyCode::Up) || keys.just_pressed(KeyCode::Down) {
        for button_data in &query {
            if button_data.id == selected_button.0 {
                if button_data.id == PLAY_BUTTON_ID {
                    selected_button.0 = QUIT_BUTTON_ID;
                } else {
                    selected_button.0 = PLAY_BUTTON_ID;
                }
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
            Interaction::Hovered => selected_button.0 = button_data.id,
            _ => {}
        }
    }
}

fn update_buttons(
    selected_button: Res<SelectedButton>,
    mut query: Query<(&UiButtonData, &mut BackgroundColor)>,
) {
    for (button_data, mut background_color) in &mut query {
        if button_data.id == selected_button.0 {
            *background_color = Color::GOLD.into();
        } else {
            *background_color = Color::NONE.into();
        }
    }
}
