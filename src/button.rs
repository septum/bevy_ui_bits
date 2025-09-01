use bevy::prelude::*;

/// Opinionated default width
const DEFAULT_BUTTON_WIDTH: Val = Val::Px(400.0);

/// Opinionated default height
const DEFAULT_BUTTON_HEIGHT: Val = Val::Px(60.0);

/// Data-holding component for the [UiButton] bundle
#[derive(Component, Default)]
pub struct UiButtonData {
    /// [usize] identifier
    pub id: usize,
    /// Optional [String] field to store data
    pub payload: Option<String>,
}

/// Button component [Bundle]
#[derive(Bundle)]
pub struct UiButton {
    node: Node,
    data: UiButtonData,
    background_color: BackgroundColor,
    marker: Button,
}

impl Default for UiButton {
    /// Creates a rectangular [UiButton]
    fn default() -> Self {
        Self {
            node: Node {
                width: DEFAULT_BUTTON_WIDTH,
                height: DEFAULT_BUTTON_HEIGHT,
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
        Self::default()
    }

    /// Creates a square [UiButton]
    pub fn square() -> Self {
        Self::default().width(DEFAULT_BUTTON_HEIGHT)
    }

    /// Sets the initial background color
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color.into();
        self
    }

    /// Sets the button width with the provided [Val]
    pub fn width(mut self, width: Val) -> Self {
        self.node.width = width;
        self
    }

    /// Sets the button height with the provided [Val]
    pub fn height(mut self, height: Val) -> Self {
        self.node.height = height;
        self
    }

    /// Sets the button ID
    pub fn id(mut self, id: usize) -> Self {
        self.data.id = id;
        self
    }

    /// Sets the button payload in [UiButtonData]
    pub fn payload(mut self, payload: &str) -> Self {
        self.data.payload = Some(payload.into());
        self
    }
}
