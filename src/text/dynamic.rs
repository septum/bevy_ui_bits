use bevy::prelude::*;

use super::UiText;

/// Data component for the [DynamicText]
#[derive(Component, Default)]
pub struct DynamicTextData {
    /// ID for the [DynamicText]
    pub id: usize,
}

/// Dynamic text builder
///
/// Use `.build()` to convert it into a [Bundle]
pub struct DynamicText {
    data: DynamicTextData,
    text: Text,
    font: TextFont,
    color: TextColor,
    initial_text: String,
}

impl Default for DynamicText {
    fn default() -> DynamicText {
        DynamicText {
            data: DynamicTextData::default(),
            text: Text::new(""),
            font: TextFont::from_font_size(super::SIZE_MEDIUM),
            color: TextColor(Color::WHITE),
            initial_text: String::new(),
        }
    }
}

impl UiText for DynamicText {
    fn color(mut self, color: TextColor) -> Self {
        self.color = color;
        self
    }

    fn font(mut self, font: TextFont) -> Self {
        self.font = font;
        self
    }

    fn text(mut self, text: Text) -> Self {
        self.text = text;
        self
    }
}

impl DynamicText {
    /// Sets the ID for the [DynamicText]
    pub fn id(mut self, id: usize) -> Self {
        self.data.id = id;
        self
    }

    /// Sets the initial value for the [DynamicText]
    pub fn initial_dynamic_text(mut self, text: &str) -> Self {
        self.initial_text = text.to_string();
        self
    }

    /// Converts [DynamicText] into a [Bundle]
    ///
    /// This will be removed in a future version aiming to
    /// make the [DynamicText] an actual [Bundle]
    pub fn build(self) -> impl Bundle {
        (
            self.data,
            self.text,
            self.font.clone(),
            self.color,
            TextLayout::new_with_justify(JustifyText::Center),
            children![(self.font, self.color, TextSpan::new(self.initial_text))],
        )
    }
}
