use bevy::{prelude::*, text::LineHeight};

/// Small font size
pub(super) const SIZE_SMALL: f32 = 20.0;

/// Medium font size
pub(super) const SIZE_MEDIUM: f32 = 40.0;

/// Large font size
pub(super) const SIZE_LARGE: f32 = 100.0;

/// Extra large font size
pub(super) const SIZE_EXTRA_LARGE: f32 = 120.0;

/// Common interface for the text bundles
pub trait UiText: Default {
    /// Create a small text variant
    fn small(value: &str, font: &Handle<Font>) -> Self {
        Self::default()
            .font(TextFont {
                font: font.clone(),
                font_size: SIZE_SMALL,
                ..default()
            })
            .text(Text::new(value))
    }

    /// Create a medium text variant
    fn medium(value: &str, font: &Handle<Font>) -> Self {
        Self::default()
            .font(TextFont {
                font: font.clone(),
                font_size: SIZE_MEDIUM,
                ..default()
            })
            .text(Text::new(value))
    }

    /// Create a large text variant
    fn large(value: &str, font: &Handle<Font>) -> Self {
        Self::default()
            .font(TextFont {
                font: font.clone(),
                font_size: SIZE_LARGE,
                ..default()
            })
            .text(Text::new(value))
    }

    /// Create a extra large text variant
    fn extra_large(value: &str, font: &Handle<Font>) -> Self {
        Self::default()
            .font(TextFont {
                font: font.clone(),
                font_size: SIZE_EXTRA_LARGE,
                ..default()
            })
            .text(Text::new(value))
    }

    /// Sets the color of the [UiText]
    fn color(self, color: TextColor) -> Self;

    /// Sets the font of the [UiText]
    fn font(self, font: TextFont) -> Self;

    /// Sets the content of the [UiText]
    fn text(self, text: Text) -> Self;

    /// Sets the line height of the [UiText]
    fn line_height(self, line_height: LineHeight) -> Self;
}
