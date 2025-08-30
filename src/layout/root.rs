use bevy::prelude::*;

const DISPERSED_CONTAINER_PADDING: UiRect = UiRect {
    top: Val::Px(19.0),
    bottom: Val::Px(28.0),
    left: Val::Px(30.0),
    right: Val::Px(28.0),
};

/// Marker component to query a [Root]
#[derive(Component)]
pub struct RootMarker;

/// Base layout [Bundle]
#[derive(Bundle)]
pub struct Root {
    node: Node,
    background_color: BackgroundColor,
    marker: RootMarker,
    transform: Transform,
    visibility: Visibility,
}

impl Default for Root {
    fn default() -> Self {
        Root {
            node: Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor::DEFAULT,
            marker: RootMarker,
            transform: Transform::default(),
            visibility: Visibility::default(),
        }
    }
}

impl Root {
    /// Creates a [Root] that congregates its content
    pub fn congregated() -> Self {
        Root::default()
    }

    /// Creates a [Root] that disperses its content
    pub fn dispersed() -> Self {
        let mut root = Root::default();
        root.node.justify_content = JustifyContent::SpaceBetween;
        root.node.padding = DISPERSED_CONTAINER_PADDING;
        root
    }

    /// Sets background color with the provided [Color]
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color.into();
        self
    }
}
