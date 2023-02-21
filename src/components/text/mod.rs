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

    /// Utility method to simplify work on multiple text sections
    fn for_each_section(&mut self, f: impl FnMut(&mut TextSection)) {
        self.text_bundle().text.sections.iter_mut().for_each(f);
    }

    /// Create a [UiText::SIZE_SMALL] variant of the implementation
    fn small<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        game_text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = Self::SIZE_SMALL;
        });

        game_text
    }

    /// Create a [UiText::SIZE_MEDIUM] variant of the implementation
    fn medium<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        game_text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = Self::SIZE_MEDIUM;
        });

        game_text
    }

    /// Create a [UiText::SIZE_LARGE] variant of the implementation
    fn large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        game_text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = Self::SIZE_LARGE;
        });

        game_text
    }

    /// Create a [UiText::SIZE_EXTRA_LARGE] variant of the implementation
    fn extra_large<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> Self {
        let mut game_text = Self::default();

        game_text.for_each_section(|section| {
            section.value = value.clone().into();
            section.style.font = font.clone();
            section.style.font_size = Self::SIZE_EXTRA_LARGE;
        });

        game_text
    }

    /// Sets color for each section of the implementation with the provided [Color]
    fn color(&mut self, color: Color) -> &mut Self {
        self.for_each_section(|section| section.style.color = color);
        self
    }

    /// Utility method that provides a mutable reference
    /// to the underlaying [TextBundle] of the implementation
    fn text_bundle(&mut self) -> &mut TextBundle;

    /// Spawns the underlaying bundle with the provided parent (mutable reference to [ChildBuilder])
    fn spawn(self, parent: &mut ChildBuilder);
}
