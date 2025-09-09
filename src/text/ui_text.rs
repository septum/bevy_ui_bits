use bevy::{prelude::*, text::LineHeight};

/// Small font size
pub(super) const DEFAULT_SIZE_SMALL: f32 = 20.0;

/// Medium font size
pub(super) const DEFAULT_SIZE_MEDIUM: f32 = 40.0;

/// Large font size
pub(super) const DEFAULT_SIZE_LARGE: f32 = 100.0;

/// Extra large font size
pub(super) const DEFAULT_SIZE_EXTRA_LARGE: f32 = 120.0;

/// Common interface for the text bundles
pub trait UiText: Default {
    /// Create a small text variant
    fn small(value: &str, font: &Handle<Font>) -> Self {
        Self::default()
            .font(font)
            .size(DEFAULT_SIZE_SMALL)
            .text(Text::new(value))
    }

    /// Create a medium text variant
    fn medium(value: &str, font: &Handle<Font>) -> Self {
        Self::default()
            .size(DEFAULT_SIZE_MEDIUM)
            .font(font)
            .text(Text::new(value))
    }

    /// Create a large text variant
    fn large(value: &str, font: &Handle<Font>) -> Self {
        Self::default()
            .size(DEFAULT_SIZE_LARGE)
            .font(font)
            .text(Text::new(value))
    }

    /// Create a extra large text variant
    fn extra_large(value: &str, font: &Handle<Font>) -> Self {
        Self::default()
            .size(DEFAULT_SIZE_EXTRA_LARGE)
            .font(font)
            .text(Text::new(value))
    }

    /// Sets the color of the [UiText]
    fn color(self, color: TextColor) -> Self;

    /// Sets the font of the [UiText]
    fn font(self, font: &Handle<Font>) -> Self;

    /// Sets the size of the [UiText]
    fn size(self, size: f32) -> Self;

    /// Sets the content of the [UiText]
    fn text(self, text: Text) -> Self;

    /// Sets the line height of the [UiText]
    fn line_height(self, line_height: LineHeight) -> Self;

    /// Sets the justification of the [UiText]
    fn justify(self, justify: JustifyText) -> Self;

    /// Sets the line break of the [UiText]
    fn line_break(self, line_break: LineBreak) -> Self;
}
