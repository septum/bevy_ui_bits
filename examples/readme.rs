use bevy::{color::palettes, prelude::*};
use bevy_ui_bits::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_ui)
        .run();
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let font = &Handle::default();

    // Root is the base layout component for a given UI tree
    let root = Root::new();

    // Container is the typical layout component
    let main_container = Container::height(Val::Px(500.0)).justify_between();

    // Customize components fluently using a builder-lite pattern
    let title = EmbossedText::large("My Game", font).color(palettes::css::GOLD.into());

    // UiButton provides a button with opinionated settings
    let play = UiButton::rectangle().background_color(palettes::css::GOLD.into());
    let play_text = EmbossedText::medium("Play", font);

    let by = SimpleText::small("By me", font);

    // Use the new Spawn API to spawn the UI tree
    commands.spawn((
        root,
        children![(
            main_container,
            children![title, (play, children![play_text]), by]
        )],
    ));
}
