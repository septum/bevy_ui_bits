use bevy::{prelude::*, text::LineHeight};

use super::UiText;

/// Embossed text component [Bundle]
#[derive(Bundle)]
pub struct EmbossedText {
    text: Text,
    font: TextFont,
    color: TextColor,
    layout: TextLayout,
    shadow: TextShadow,
}

impl Default for EmbossedText {
    fn default() -> Self {
        Self {
            text: Text::new(""),
            font: TextFont::from_font_size(super::DEFAULT_SIZE_MEDIUM),
            color: TextColor(Color::WHITE),
            layout: TextLayout::new_with_justify(JustifyText::Center),
            shadow: TextShadow {
                offset: Vec2::splat(2.0),
                color: Color::BLACK,
            },
        }
    }
}

impl UiText for EmbossedText {
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

impl EmbossedText {
    /// Create a small text variant
    pub fn small(value: &str, font: &Handle<Font>) -> Self {
        <Self as UiText>::small(value, font).relief(1.0)
    }

    /// Create a medium text variant
    pub fn medium(value: &str, font: &Handle<Font>) -> Self {
        <Self as UiText>::medium(value, font).relief(2.0)
    }

    /// Create a large text variant
    pub fn large(value: &str, font: &Handle<Font>) -> Self {
        <Self as UiText>::large(value, font).relief(5.0)
    }

    /// Create a extra large text variant
    pub fn extra_large(value: &str, font: &Handle<Font>) -> Self {
        <Self as UiText>::extra_large(value, font).relief(6.0)
    }

    /// Sets the shadow color of the text
    pub fn shadow(mut self, color: Color) -> Self {
        self.shadow.color = color;
        self
    }

    /// Sets the offset of the text shadow with the provided relief
    fn relief(mut self, relief: f32) -> Self {
        self.shadow.offset = Vec2::splat(relief);
        self
    }
}
