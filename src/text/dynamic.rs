use bevy::prelude::*;

use super::UiText;

/// Data-holding component for the [DynamicText] bundle
#[derive(Component, Default)]
pub struct DynamicTextData {
    /// [usize] identifier
    pub id: usize,
}

/// Dynamic text [Bundle] builder
///
/// Use `.build()` to convert it into a [Bundle] to spawn it
pub struct DynamicTextBuilder {
    data: DynamicTextData,
    text: Text,
    font: TextFont,
    color: TextColor,
    initial_text: String,
}

impl Default for DynamicTextBuilder {
    fn default() -> Self {
        Self {
            data: DynamicTextData::default(),
            text: Text::new(""),
            font: TextFont::from_font_size(super::SIZE_MEDIUM),
            color: TextColor(Color::WHITE),
            initial_text: String::new(),
        }
    }
}

impl UiText for DynamicTextBuilder {
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

impl DynamicTextBuilder {
    /// Sets the text ID
    pub fn id(mut self, id: usize) -> Self {
        self.data.id = id;
        self
    }

    /// Sets the initial dynamic text value
    pub fn initial_dynamic_text(mut self, text: &str) -> Self {
        self.initial_text = text.to_string();
        self
    }

    /// Builds [DynamicText] into a [Bundle]
    ///
    /// This will take the `initial_text` field, wrap it in a [TextSpan], and set it
    /// as a child of the bundle, so it can be queried and modified by a system
    ///
    /// **NOTE:** This will be removed in a future version aiming to
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
