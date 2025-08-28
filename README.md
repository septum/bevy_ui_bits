# BEVY UI BITS

[![crates.io](https://img.shields.io/crates/v/bevy_ui_bits)](https://crates.io/crates/bevy_ui_bits)
[![license](https://img.shields.io/crates/l/bevy_ui_bits)](https://crates.io/crates/bevy_ui_bits)

A mingy and opinionated collection of UI components for Bevy

## Usage

```rust
use bevy::prelude::*;
use bevy_ui_bits::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_ui)
        .run();
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let font = &Handle::default();

    // Root is the encompassing component for a given UI tree
    let root = Root::congregated();

    // Container is the quintessential layout component
    let mut main_container = Container::height(400.0);

    // EmbossedText represents text with a background relief
    let mut title = EmbossedText::large("My Game", font);

    // UiButton wraps over a ButtonBundle with opinionated defaults
    let mut play = UiButton::new("Start", font);

    let by = SimpleText::small("By me", font);

    // Make changes to the properties with a fluent interface
    main_container.justify_between();
    title.color(Color::YELLOW);
    play.selected_color(Color::MIDNIGHT_BLUE);

    // Use a nested structure to spawn the UI tree
    root.spawn(&mut commands, |parent| {
        main_container.spawn(parent, |parent| {
            title.spawn(parent);
            play.spawn(parent);
            by.spawn(parent);
        });
    });
}
```

Try it out with:

```
cargo run --example readme --features="bevy/default"
```

## Examples

### Basic main menu UI that supports both mouse and keyboard input

![main_menu](https://user-images.githubusercontent.com/4467518/220443135-350551c0-2af4-4f54-b436-73d83647fc66.png)

Try it out with:

```
cargo run --example main_menu --features="bevy/default"
```

### Simple HUD that features a dynamic text component

![hud](https://user-images.githubusercontent.com/4467518/220443052-18ebaf41-d857-495e-9dd9-c38dd8878440.png)

Try it out with:

```
cargo run --example hud --features="bevy/default"
```

### Despawn UI recursively with the `RootMarker`

![despawn](https://github.com/septum/bevy_ui_bits/assets/4467518/a873033b-f709-4c15-8842-a93829ccd483)

Try it out with:

```
cargo run --example despawn --features="bevy/default"
```

## Bevy Compatibility

| bevy | bevy_ui_bits |
| ---- | ------------ |
| 0.13 | 0.5          |
| 0.12 | 0.4          |
| 0.11 | 0.3          |
| 0.10 | 0.2          |
| 0.9  | 0.1          |

## License

This project is dual-licensed under either:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

at your option.

With the exception of the Fira Mono Font, which has its own license.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
