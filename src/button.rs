use bevy::prelude::*;

/// Data component for the [UiButton]
#[derive(Component, Default)]
pub struct UiButtonData {
    /// ID for the [UiButton]
    pub id: usize,
    /// Optional [String] value field to store data
    pub value: Option<String>,
}

/// Button [Bundle] that holds [UiButtonData]
#[derive(Bundle)]
pub struct UiButton {
    node: Node,
    data: UiButtonData,
    background_color: BackgroundColor,
    marker: Button,
}

impl Default for UiButton {
    fn default() -> UiButton {
        UiButton {
            node: Node {
                width: Val::Px(400.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            data: UiButtonData::default(),
            background_color: BackgroundColor::DEFAULT,
            marker: Button,
        }
    }
}

impl UiButton {
    /// Creates a rectangular [UiButton]
    pub fn rectangle() -> Self {
        UiButton::default()
    }

    /// Creates a square [UiButton]
    pub fn square() -> Self {
        let mut button = Self::default();
        button.node.width = Val::Px(60.0);
        button
    }

    /// Sets an initial background color with the provided [Color]
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color.into();
        self
    }

    /// Sets width in pixels
    pub fn width(mut self, width: f32) -> Self {
        self.node.width = Val::Px(width);
        self
    }

    /// Sets height in pixels
    pub fn height(mut self, height: f32) -> Self {
        self.node.height = Val::Px(height);
        self
    }

    /// Sets id of the [UiButton]
    pub fn id(mut self, id: usize) -> Self {
        self.data.id = id;
        self
    }

    /// Sets payload of the [UiButton]
    pub fn data(mut self, payload: &str) -> Self {
        self.data.value = Some(payload.into());
        self
    }
}
