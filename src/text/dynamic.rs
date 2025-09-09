use bevy::{prelude::*, text::LineHeight};

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
    layout: TextLayout,
    dynamic_text: String,
}

impl Default for DynamicTextBuilder {
    fn default() -> Self {
        Self {
            data: DynamicTextData::default(),
            text: Text::new(""),
            font: TextFont::from_font_size(super::DEFAULT_SIZE_MEDIUM),
            color: TextColor(Color::WHITE),
            layout: TextLayout::new_with_justify(JustifyText::Center),
            dynamic_text: String::new(),
        }
    }
}

impl UiText for DynamicTextBuilder {
    fn color(mut self, color: TextColor) -> Self {
        self.color = color;
        self
    }

    fn font(mut self, font: &Handle<Font>) -> Self {
        self.font.font = font.clone();
        self
    }

    fn size(mut self, size: f32) -> Self {
        self.font.font_size = size;
        self
    }

    fn text(mut self, text: Text) -> Self {
        self.text = text;
        self
    }

    fn line_height(mut self, line_height: LineHeight) -> Self {
        self.font.line_height = line_height;
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
        self.dynamic_text = text.to_string();
        self
    }

    /// Builds [DynamicText] into a [Bundle]
    ///
    /// This will take the `dynamic_text` field, wrap it in a [TextSpan], and set it
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
            self.layout,
            children![(self.font, self.color, TextSpan::new(self.dynamic_text))],
        )
    }
}
