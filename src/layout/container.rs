use bevy::prelude::*;

/// Typical layout component [Bundle]
#[derive(Bundle)]
pub struct Container {
    node: Node,
    background_color: BackgroundColor,
    transform: Transform,
    visibility: Visibility,
}

impl Default for Container {
    /// Creates a new [Container] with:
    /// - auto width and height,
    /// - centered content and items,
    /// - and column flex direction.
    fn default() -> Self {
        Container {
            node: Node {
                height: Val::Auto,
                width: Val::Auto,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor::DEFAULT,
            transform: Transform::default(),
            visibility: Visibility::default(),
        }
    }
}

impl Container {
    /// Creates a new [Container] with:
    /// - auto width and height,
    /// - centered content and items,
    /// - and column flex direction.
    pub fn new() -> Self {
        Container::default()
    }

    /// Creates a [Container] with the provided width value
    pub fn width(width: Val) -> Self {
        let mut container = Self::default();
        container.node.width = width;
        container
    }

    /// Creates a [Container] with the provided height value
    pub fn height(height: Val) -> Self {
        let mut container = Self::default();
        container.node.height = height;
        container
    }

    /// Creates a [Container] with the provided width and height values
    pub fn size(width: Val, height: Val) -> Self {
        let mut container = Self::default();
        container.node.width = width;
        container.node.height = height;
        container
    }

    /// Sets the background color with the provided [Color]
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color.into();
        self
    }

    /// Sets position type as [PositionType::Absolute]
    pub fn absolute(mut self) -> Self {
        self.node.position_type = PositionType::Absolute;
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

    /// Sets align content as [AlignContent::FlexStart]
    pub fn content_start(mut self) -> Self {
        self.node.align_content = AlignContent::FlexStart;
        self
    }

    /// Sets flex wrap as [FlexWrap::Wrap]
    pub fn wrap(mut self) -> Self {
        self.node.flex_wrap = FlexWrap::Wrap;
        self
    }

    /// Sets the container padding with the provided [UiRect]
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.node.padding = padding;
        self
    }

    /// Sets the container margin with the provided [UiRect]
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.node.margin = margin;
        self
    }
}
