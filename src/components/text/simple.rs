use bevy::prelude::*;

use super::UiText;

/// Simple text component
#[derive(Bundle)]
pub struct SimpleText {
    text: Text,
    font: TextFont,
    color: TextColor,
    layout: TextLayout,
}

impl Default for SimpleText {
    fn default() -> SimpleText {
        let font = TextFont {
            font: Handle::default(),
            font_size: SimpleText::SIZE_MEDIUM,
            ..default()
        };
        let color = TextColor(Color::WHITE);
        let text = Text::new("");
        let layout = TextLayout::new_with_justify(JustifyText::Center);

        SimpleText {
            text,
            font,
            color,
            layout,
        }
    }
}

impl UiText for SimpleText {
    fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn(self);
    }

    fn color(&mut self, color: TextColor) {
        self.color = color;
    }

    fn font(&mut self, font: TextFont) {
        self.font = font;
    }

    fn text(&mut self, text: Text) {
        self.text = text;
    }
}
