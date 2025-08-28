mod dynamic;
mod embossed;
mod simple;

pub use dynamic::{DynamicText, DynamicTextData};
pub use embossed::EmbossedText;
pub use simple::SimpleText;

use bevy::prelude::*;

/// Text common interface
pub trait UiText: Default {
    /// Small text size
    const SIZE_SMALL: f32 = 18.0;

    /// Medium text size
    const SIZE_MEDIUM: f32 = 36.0;

    /// Large text size
    const SIZE_LARGE: f32 = 90.0;

    /// Extra large text size
    const SIZE_EXTRA_LARGE: f32 = 108.0;

    /// Create a [UiText::SIZE_SMALL] variant of the implementation
    fn small<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut ui_text = Self::default();

        ui_text.font(TextFont {
            font: font.clone(),
            font_size: Self::SIZE_SMALL,
            ..default()
        });
        ui_text.text(Text::new(value));
        ui_text
    }

    /// Create a [UiText::SIZE_MEDIUM] variant of the implementation
    fn medium<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut ui_text = Self::default();

        ui_text.font(TextFont {
            font: font.clone(),
            font_size: Self::SIZE_MEDIUM,
            ..default()
        });
        ui_text.text(Text::new(value));
        ui_text
    }

    /// Create a [UiText::SIZE_LARGE] variant of the implementation
    fn large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut ui_text = Self::default();

        ui_text.font(TextFont {
            font: font.clone(),
            font_size: Self::SIZE_LARGE,
            ..default()
        });
        ui_text.text(Text::new(value));

        ui_text
    }

    /// Create a [UiText::SIZE_EXTRA_LARGE] variant of the implementation
    fn extra_large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut ui_text = Self::default();

        ui_text.font(TextFont {
            font: font.clone(),
            font_size: Self::SIZE_EXTRA_LARGE,
            ..default()
        });
        ui_text.text(Text::new(value));

        ui_text
    }

    /// Get a mut ref of the color of the component
    fn color(&mut self, color: TextColor);

    /// Get a mut ref of the font of the component
    fn font(&mut self, font: TextFont);

    /// Get a mut ref of the inner text component
    fn text(&mut self, text: Text);

    /// Spawns the underlaying bresetundle with the provided parent (mutable reference to [ChildBuilder])
    fn spawn(self, parent: &mut ChildSpawnerCommands);
}
