use bevy::prelude::*;

const DISPERSED_CONTAINER_PADDING: UiRect = UiRect {
    top: Val::Px(19.0),
    bottom: Val::Px(28.0),
    left: Val::Px(30.0),
    right: Val::Px(28.0),
};

/// Marker component to query a [Root] component
#[derive(Component)]
pub struct RootMarker;

/// Top parent component that acts as a UI overlay
pub struct Root {
    bundle: NodeBundle,
}

impl Default for Root {
    fn default() -> Root {
        Root {
            bundle: NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
        }
    }
}

impl Root {
    /// Creates a [Root] that congregates its content
    ///
    /// This equivalent to calling `Root::default()`
    pub fn congregated() -> Root {
        Root::default()
    }

    /// Creates a [Root] that disperses its content
    pub fn dispersed() -> Root {
        let mut root = Root::default();
        root.bundle.style.justify_content = JustifyContent::SpaceBetween;
        root.bundle.style.padding = DISPERSED_CONTAINER_PADDING;
        root
    }

    /// Sets background color with the provided [Color]
    pub fn background_color(&mut self, color: Color) -> &mut Root {
        self.bundle.background_color = color.into();
        self
    }

    /// Spawns the underlaying bundle with the given children built in the given closure
    ///
    /// [Root] additionaly takes a mutable reference to [Commands]
    pub fn spawn(self, commands: &mut Commands, spawn_children: impl FnOnce(&mut ChildBuilder)) {
        commands
            .spawn(self.bundle)
            .with_children(spawn_children)
            .insert(RootMarker);
    }
}
