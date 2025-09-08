use bevy::prelude::*;

/// Marker component to query a [Root]
#[derive(Component)]
pub struct RootMarker;

/// Base layout component [Bundle] for a UI tree
#[derive(Bundle)]
pub struct Root {
    node: Node,
    background_color: BackgroundColor,
    marker: RootMarker,
    transform: Transform,
    visibility: Visibility,
}

impl Default for Root {
    /// Creates a new [Root] with:
    /// - full width and height,
    /// - centered content and items,
    /// - and column flex direction.
    fn default() -> Self {
        Self {
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
    /// Creates a new [Root] with:
    /// - full width and height,
    /// - centered content and items,
    /// - and column flex direction.
    pub fn new() -> Self {
        Root::default()
    }

    /// Sets the background color with the provided [Color]
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color.into();
        self
    }

    /// Sets flex direction as [FlexDirection::Row]
    pub fn row(mut self) -> Self {
        self.node.flex_direction = FlexDirection::Row;
        self
    }

    /// Sets justify content as [JustifyContent::FlexStart]
    pub fn justify_start(mut self) -> Self {
        self.node.justify_content = JustifyContent::FlexStart;
        self
    }

    /// Sets justify content as [JustifyContent::FlexEnd]
    pub fn justify_end(mut self) -> Self {
        self.node.justify_content = JustifyContent::FlexEnd;
        self
    }

    /// Sets justify content as [JustifyContent::SpaceBetween]
    pub fn justify_between(mut self) -> Self {
        self.node.justify_content = JustifyContent::SpaceBetween;
        self
    }

    /// Sets justify content as [JustifyContent::SpaceAround]
    pub fn justify_around(mut self) -> Self {
        self.node.justify_content = JustifyContent::SpaceAround;
        self
    }

    /// Sets align items as [AlignItems::FlexStart]
    pub fn items_start(mut self) -> Self {
        self.node.align_items = AlignItems::FlexStart;
        self
    }

    /// Sets align items as [AlignItems::FlexEnd]
    pub fn items_end(mut self) -> Self {
        self.node.align_items = AlignItems::FlexEnd;
        self
    }

    /// Sets the container padding with the provided [UiRect]
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.node.padding = padding;
        self
    }
}
