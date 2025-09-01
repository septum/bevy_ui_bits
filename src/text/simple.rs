use bevy::prelude::*;

use super::UiText;

/// Simple text component [Bundle]
#[derive(Bundle)]
pub struct SimpleText {
    text: Text,
    font: TextFont,
    color: TextColor,
    layout: TextLayout,
}

impl Default for SimpleText {
    fn default() -> Self {
        Self {
            text: Text::new(""),
            font: TextFont::from_font_size(super::SIZE_MEDIUM),
            color: TextColor(Color::WHITE),
            layout: TextLayout::new_with_justify(JustifyText::Center),
        }
    }
}

impl UiText for SimpleText {
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
