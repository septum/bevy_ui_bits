use bevy::prelude::*;

use super::UiText;

/// Inner bundle of component
#[derive(Bundle, Clone)]
pub struct EmbossedTextBundle {
    text: Text,
    font: TextFont,
    color: TextColor,
    layout: TextLayout,
    node: Node,
}

/// Embossed text component
pub struct EmbossedText {
    bundle: EmbossedTextBundle,
    background_color: TextColor,
}

impl Default for EmbossedText {
    fn default() -> EmbossedText {
        let font = TextFont {
            font: Handle::default(),
            font_size: EmbossedText::SIZE_MEDIUM,
            ..default()
        };
        let color = TextColor(Color::WHITE);
        let background_color = TextColor(Color::BLACK);
        let text = Text::new("");
        let layout = TextLayout::new_with_justify(JustifyText::Center);
        let bundle = EmbossedTextBundle {
            text,
            font,
            color,
            layout,
            node: Node::default(),
        };

        EmbossedText {
            bundle,
            background_color,
        }
    }
}

impl UiText for EmbossedText {
    fn spawn(self, parent: &mut ChildSpawnerCommands) {
        let mut foreground = self.bundle.clone();
        let mut background = self.bundle;
        let relief = foreground.font.font_size / EmbossedText::SIZE_SMALL;

        foreground.node.position_type = PositionType::Absolute;
        background.node.top = Val::Px(relief);
        background.node.left = Val::Px(relief);
        background.color = self.background_color;

        parent.spawn(background);
        parent.spawn(foreground);
    }

    fn color(&mut self, color: TextColor) {
        self.bundle.color = color;
    }

    fn font(&mut self, font: TextFont) {
        self.bundle.font = font;
    }

    fn text(&mut self, text: Text) {
        self.bundle.text = text;
    }
}

impl EmbossedText {
    /// Sets the background color
    pub fn background_color(&mut self, background_color: TextColor) {
        self.background_color = background_color;
    }
}
