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

    // Root is the base component for a given UI tree
    let root = Root::congregated();

    // Container is the common layout component
    let mut main_container = Container::height(400.0);

    // EmbossedText represents text with a background relief
    let mut title = EmbossedText::large("My Game", font);

    // UiButton wraps over a ButtonBundle with opinionated defaults
    let mut play = UiButton::new("Start", font);

    let by = SimpleText::medium("By me", font);

    // Make changes to the properties with a fluent interface
    main_container.justify_between();
    title.color(palettes::css::GOLD.into());
    title.background_color(palettes::css::MIDNIGHT_BLUE.into());
    play.selected_color(palettes::css::MIDNIGHT_BLUE.into());

    // Use a nested structure to spawn the UI tree
    root.spawn(&mut commands, |parent| {
        main_container.spawn(parent, |parent| {
            title.spawn(parent);
            play.spawn(parent);
            by.spawn(parent);
        });
    });
}
