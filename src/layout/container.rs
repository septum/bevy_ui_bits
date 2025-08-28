use bevy::prelude::*;

/// Fundamental layout component
pub struct Container {
    node: Node,
    background_color: BackgroundColor,
}

impl Default for Container {
    fn default() -> Container {
        Container {
            node: Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
        }
    }
}

impl Container {
    /// Creates a [Container] with the provided width in pixels
    pub fn width(width: f32) -> Container {
        let mut container = Self::default();
        container.node.width = Val::Px(width);
        container
    }

    /// Creates a [Container] with the provided height in pixels
    pub fn height(height: f32) -> Container {
        let mut container = Self::default();
        container.node.height = Val::Px(height);
        container
    }

    /// Creates a [Container] with the provided width and height in pixels
    pub fn size(width: f32, height: f32) -> Container {
        let mut container = Self::default();
        container.node.width = Val::Px(width);
        container.node.height = Val::Px(height);
        container
    }

    /// Creates a [Container] with the provided width and height in percent
    pub fn size_percentage(width: f32, height: f32) -> Container {
        let mut container = Self::default();
        container.node.width = Val::Percent(width);
        container.node.height = Val::Percent(height);
        container
    }

    /// Creates a [Container] with the width and height automatically determined
    pub fn auto() -> Container {
        let mut container = Self::default();
        container.node.width = Val::Auto;
        container.node.height = Val::Auto;
        container
    }

    /// Creates a [Container] with the height automatically determined
    pub fn auto_height() -> Container {
        let mut container = Self::default();
        container.node.height = Val::Auto;
        container
    }

    /// Creates a [Container] with the height automatically determined
    /// and the provided width in pixels
    pub fn auto_height_with_width(width: f32) -> Container {
        let mut container = Self::auto_height();
        container.node.width = Val::Px(width);
        container
    }

    /// Creates a [Container] with 50% of its parent width
    pub fn half_width() -> Container {
        let mut container = Self::default();
        container.node.width = Val::Percent(50.0);
        container
    }

    /// Sets position type as [PositionType::Absolute]
    pub fn absolute(&mut self) -> &mut Container {
        self.node.position_type = PositionType::Absolute;
        self
    }

    /// Sets flex direction as [FlexDirection::Row]
    pub fn row(&mut self) -> &mut Container {
        self.node.flex_direction = FlexDirection::Row;
        self
    }

    /// Sets justify content as [JustifyContent::SpaceBetween]
    pub fn justify_between(&mut self) -> &mut Container {
        self.node.justify_content = JustifyContent::SpaceBetween;
        self
    }

    /// Sets justify content as [JustifyContent::SpaceAround]
    pub fn justify_around(&mut self) -> &mut Container {
        self.node.justify_content = JustifyContent::SpaceAround;
        self
    }

    /// Sets justify content as [JustifyContent::FlexStart]
    pub fn justify_start(&mut self) -> &mut Container {
        self.node.justify_content = JustifyContent::FlexStart;
        self
    }

    /// Sets justify content as [JustifyContent::FlexEnd]
    pub fn justify_end(&mut self) -> &mut Container {
        self.node.justify_content = JustifyContent::FlexEnd;
        self
    }

    /// Sets align items as [AlignItems::FlexStart]
    pub fn items_start(&mut self) -> &mut Container {
        self.node.align_items = AlignItems::FlexStart;
        self
    }

    /// Sets align content as [AlignContent::FlexStart]
    pub fn content_start(&mut self) -> &mut Container {
        self.node.align_content = AlignContent::FlexStart;
        self
    }

    /// Sets flex wrap as [FlexWrap::Wrap]
    pub fn wrap(&mut self) -> &mut Container {
        self.node.flex_wrap = FlexWrap::Wrap;
        self
    }

    /// Sets align items as [AlignItems::FlexStart]
    pub fn align_start(&mut self) -> &mut Container {
        self.node.align_items = AlignItems::FlexStart;
        self
    }

    /// Sets align items as [AlignItems::FlexEnd]
    pub fn align_end(&mut self) -> &mut Container {
        self.node.align_items = AlignItems::FlexEnd;
        self
    }

    /// Sets the bottom margin with the provided margin as pixels
    pub fn bottom_margin(&mut self, margin: f32) -> &mut Container {
        self.node.margin.bottom = Val::Px(margin);
        self
    }

    /// Sets background color with the provided [Color]
    pub fn background_color(&mut self, color: Color) -> &mut Container {
        self.background_color = color.into();
        self
    }

    /// Spawns the underlaying bundle with the given children built in the given closure
    pub fn spawn(
        self,
        parent: &mut ChildSpawnerCommands,
        spawn_children: impl FnOnce(&mut ChildSpawnerCommands),
    ) {
        parent
            .spawn((self.node, self.background_color))
            .with_children(spawn_children);
    }
}
