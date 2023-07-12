use bevy::prelude::*;

/// Fundamental layout component
pub struct Container {
    bundle: NodeBundle,
}

impl Default for Container {
    fn default() -> Container {
        Container {
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

impl Container {
    /// Creates a [Container] with the provided width in pixels
    pub fn width(width: f32) -> Container {
        let mut housing = Self::default();
        housing.bundle.style.width = Val::Px(width);
        housing
    }

    /// Creates a [Container] with the provided height in pixels
    pub fn height(height: f32) -> Container {
        let mut housing = Self::default();
        housing.bundle.style.height = Val::Px(height);
        housing
    }

    /// Creates a [Container] with the provided width and height in pixels
    pub fn size(width: f32, height: f32) -> Container {
        let mut housing = Self::default();
        housing.bundle.style.width = Val::Px(width);
        housing.bundle.style.height = Val::Px(height);
        housing
    }

    /// Creates a [Container] with the provided width and height in percent
    pub fn size_percentage(width: f32, height: f32) -> Container {
        let mut housing = Self::default();
        housing.bundle.style.width = Val::Percent(width);
        housing.bundle.style.height = Val::Percent(height);
        housing
    }

    /// Creates a [Container] with the width and height automatically determined
    pub fn auto() -> Container {
        let mut housing = Self::default();
        housing.bundle.style.width = Val::Auto;
        housing.bundle.style.height = Val::Auto;
        housing
    }

    /// Creates a [Container] with the height automatically determined
    pub fn auto_height() -> Container {
        let mut housing = Self::default();
        housing.bundle.style.height = Val::Auto;
        housing
    }

    /// Creates a [Container] with the height automatically determined
    /// and the provided width in pixels
    pub fn auto_height_with_width(width: f32) -> Container {
        let mut housing = Self::auto_height();
        housing.bundle.style.width = Val::Px(width);
        housing
    }

    /// Creates a [Container] with 50% of its parent width
    pub fn half_width() -> Container {
        let mut housing = Self::default();
        housing.bundle.style.width = Val::Percent(50.0);
        housing
    }

    /// Sets position type as [PositionType::Absolute]
    pub fn absolute(&mut self) -> &mut Container {
        self.bundle.style.position_type = PositionType::Absolute;
        self
    }

    /// Sets flex direction as [FlexDirection::Row]
    pub fn row(&mut self) -> &mut Container {
        self.bundle.style.flex_direction = FlexDirection::Row;
        self
    }

    /// Sets justify content as [JustifyContent::SpaceBetween]
    pub fn justify_between(&mut self) -> &mut Container {
        self.bundle.style.justify_content = JustifyContent::SpaceBetween;
        self
    }

    /// Sets justify content as [JustifyContent::SpaceAround]
    pub fn justify_around(&mut self) -> &mut Container {
        self.bundle.style.justify_content = JustifyContent::SpaceAround;
        self
    }

    /// Sets justify content as [JustifyContent::FlexStart]
    pub fn justify_start(&mut self) -> &mut Container {
        self.bundle.style.justify_content = JustifyContent::FlexStart;
        self
    }

    /// Sets justify content as [JustifyContent::FlexEnd]
    pub fn justify_end(&mut self) -> &mut Container {
        self.bundle.style.justify_content = JustifyContent::FlexEnd;
        self
    }

    /// Sets align items as [AlignItems::FlexStart]
    pub fn items_start(&mut self) -> &mut Container {
        self.bundle.style.align_items = AlignItems::FlexStart;
        self
    }

    /// Sets align content as [AlignContent::FlexStart]
    pub fn content_start(&mut self) -> &mut Container {
        self.bundle.style.align_content = AlignContent::FlexStart;
        self
    }

    /// Sets flex wrap as [FlexWrap::Wrap]
    pub fn wrap(&mut self) -> &mut Container {
        self.bundle.style.flex_wrap = FlexWrap::Wrap;
        self
    }

    /// Sets align items as [AlignItems::FlexStart]
    pub fn align_start(&mut self) -> &mut Container {
        self.bundle.style.align_items = AlignItems::FlexStart;
        self
    }

    /// Sets align items as [AlignItems::FlexEnd]
    pub fn align_end(&mut self) -> &mut Container {
        self.bundle.style.align_items = AlignItems::FlexEnd;
        self
    }

    /// Sets the bottom margin with the provided margin as pixels
    pub fn bottom_margin(&mut self, margin: f32) -> &mut Container {
        self.bundle.style.margin.bottom = Val::Px(margin);
        self
    }

    /// Sets background color with the provided [Color]
    pub fn background_color(&mut self, color: Color) -> &mut Container {
        self.bundle.background_color = color.into();
        self
    }

    /// Spawns the underlaying bundle with the given children built in the given closure
    pub fn spawn(self, parent: &mut ChildBuilder, spawn_children: impl FnOnce(&mut ChildBuilder)) {
        parent.spawn(self.bundle).with_children(spawn_children);
    }
}
