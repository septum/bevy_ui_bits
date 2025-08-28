use bevy::prelude::*;

use super::UiText;

/// Data for the dynamic text component
#[derive(Component, Default)]
pub struct DynamicTextData {
    /// ID of the text
    pub id: usize,
}

/// Dynamic text component
pub struct DynamicText {
    data: DynamicTextData,
    text: Text,
    span: TextSpan,
    font: TextFont,
    color: TextColor,
    layout: TextLayout,
}

impl Default for DynamicText {
    fn default() -> DynamicText {
        let font = TextFont {
            font: Handle::default(),
            font_size: DynamicText::SIZE_MEDIUM,
            ..default()
        };
        let color = TextColor(Color::WHITE);
        let text = Text::new("");
        let span = TextSpan::new("");
        let layout = TextLayout::new_with_justify(JustifyText::Center);

        DynamicText {
            data: DynamicTextData::default(),
            text,
            span,
            font,
            color,
            layout,
        }
    }
}

impl UiText for DynamicText {
    fn spawn(self, parent: &mut ChildSpawnerCommands) {
        parent
            .spawn((
                self.text,
                self.font.clone(),
                self.color,
                self.layout,
                self.data,
            ))
            .with_child((self.span, self.font, self.color, self.layout));
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

impl DynamicText {
    /// Method to change the id of the text component
    pub fn id(&mut self, id: usize) -> &mut DynamicText {
        self.data.id = id;
        self
    }

    /// Set the initial dynamic text value
    pub fn dynamic_text_value<S: Into<String> + Clone>(&mut self, text: S) -> &mut DynamicText {
        self.span.0 = text.into();
        self
    }
}
