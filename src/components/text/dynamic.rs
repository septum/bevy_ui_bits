use bevy::prelude::*;

use super::UiText;

/// Dynamic text component
pub struct DynamicText {
    data: DynamicTextData,
    bundle: TextBundle,
}

/// Marker component with an id associated to a [DynamicText]
#[derive(Component, Default)]
pub struct DynamicTextData {
    /// [DynamicText] identifier
    pub id: usize,
}

impl Default for DynamicText {
    fn default() -> DynamicText {
        let section = TextSection {
            value: String::new(),
            style: TextStyle {
                font: Handle::default(),
                font_size: DynamicText::SIZE_MEDIUM,
                color: Color::WHITE,
            },
        };
        DynamicText {
            data: DynamicTextData::default(),
            bundle: TextBundle::from_sections(vec![section; 2])
                .with_text_alignment(TextAlignment::CENTER),
        }
    }
}

impl UiText for DynamicText {
    fn text_bundle(&mut self) -> &mut TextBundle {
        &mut self.bundle
    }

    fn spawn(self, parent: &mut ChildBuilder) {
        parent.spawn(self.bundle).insert(self.data);
    }
}

impl DynamicText {
    /// Sets id of [DynamicTextData] with the provided id
    pub fn id(&mut self, id: usize) -> &mut DynamicText {
        self.data.id = id;
        self
    }

    /// Sets a default dynamic value
    pub fn dynamic_text_value<S: Into<String> + Clone>(&mut self, text: S) -> &mut DynamicText {
        self.bundle.text.sections[1].value = text.into();
        self
    }
}
