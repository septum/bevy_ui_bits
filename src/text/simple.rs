use bevy::{prelude::*, text::LineHeight};

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
            font: TextFont::from_font_size(super::DEFAULT_SIZE_MEDIUM),
            color: TextColor(Color::WHITE),
            layout: TextLayout::new_with_justify(Justify::Center),
        }
    }
}

impl UiText for SimpleText {
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

    fn justify(mut self, justify: Justify) -> Self {
        self.layout.justify = justify;
        self
    }

    fn line_break(mut self, line_break: LineBreak) -> Self {
        self.layout.linebreak = line_break;
        self
    }
}
