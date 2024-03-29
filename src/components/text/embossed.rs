use bevy::prelude::*;

use super::UiText;

/// Embossed text component
pub struct EmbossedText {
    bundle: TextBundle,
    background_color: Color,
}

impl Default for EmbossedText {
    fn default() -> EmbossedText {
        let style = TextStyle {
            font: Handle::default(),
            font_size: EmbossedText::SIZE_MEDIUM,
            color: Color::WHITE,
        };
        EmbossedText {
            bundle: TextBundle::from_section("", style).with_text_alignment(TextAlignment::Center),
            background_color: Color::BLACK,
        }
    }
}

impl UiText for EmbossedText {
    fn text_bundle(&mut self) -> &mut TextBundle {
        &mut self.bundle
    }

    fn spawn(self, parent: &mut ChildBuilder) {
        let style = self.bundle.text.sections[0].style.clone();
        let mut background =
            TextBundle::from_section("", style).with_text_alignment(TextAlignment::Center);
        let mut foreground = self.bundle;
        let relief = foreground.text.sections[0].style.font_size / EmbossedText::SIZE_SMALL;

        foreground.style.position_type = PositionType::Absolute;
        background.style.top = Val::Px(relief);
        background.style.left = Val::Px(relief);
        background.text.sections[0].style.color = self.background_color;

        parent.spawn(background);
        parent.spawn(foreground);
    }
}

impl EmbossedText {
    /// Sets background color with the provided [Color]
    pub fn background_color(&mut self, color: Color) -> &mut EmbossedText {
        self.background_color = color;
        self
    }
}
