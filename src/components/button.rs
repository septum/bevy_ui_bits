use bevy::prelude::*;

use super::text::{EmbossedText, UiText};

/// Button component that holds [UiButtonData]
pub struct UiButton {
    node: Node,
    background_color: BackgroundColor,
    child: EmbossedText,
    data: UiButtonData,
}

/// Marker component with data associated to a [UiButton]
#[derive(Component, Default)]
pub struct UiButtonData {
    /// [UiButton] identified
    pub id: usize,
    /// Optional [String] payload
    pub payload: Option<String>,
}

impl Default for UiButton {
    fn default() -> UiButton {
        UiButton {
            node: Node {
                width: Val::Px(400.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            child: EmbossedText::default(),
            data: UiButtonData::default(),
        }
    }
}

impl UiButton {
    /// Creates a [UiButton] with the provided string value and font handle
    pub fn new<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> UiButton {
        UiButton {
            child: EmbossedText::medium(value, font),
            ..default()
        }
    }

    /// Creates a square [UiButton] with the provided string value and font handle
    pub fn square<S: Into<String> + Clone>(value: S, font: &Handle<Font>) -> UiButton {
        let mut button = Self::default();
        button.node.width = Val::Px(60.0);
        button.node.height = Val::Px(60.0);
        button.child = EmbossedText::medium(value, font);
        button
    }

    /// Sets text color with the provided [Color]
    pub fn color(&mut self, color: Color) -> &mut UiButton {
        self.child.color(color.into());
        self
    }

    /// Sets background color with the provided [Color]
    pub fn selected_color(&mut self, color: Color) -> &mut UiButton {
        self.background_color = color.into();
        self
    }

    /// Sets width with the provided width in pixels
    pub fn width(&mut self, width: f32) -> &mut UiButton {
        self.node.width = Val::Px(width);
        self
    }

    /// Sets height with the provided height in pixels
    pub fn height(&mut self, height: f32) -> &mut UiButton {
        self.node.height = Val::Px(height);
        self
    }

    /// Sets id of [UiButtonData] with the provided id
    pub fn id(&mut self, id: usize) -> &mut UiButton {
        self.data.id = id;
        self
    }

    /// Sets payload of [UiButtonData]
    pub fn payload<S: Into<String> + Clone>(&mut self, payload: S) -> &mut UiButton {
        self.data.payload = Some(payload.into());
        self
    }

    /// Spawns the underlaying bundle with the provided parent (mutable reference to [ChildBuilder])
    pub fn spawn(self, parent: &mut ChildSpawnerCommands) {
        parent
            .spawn((self.node, self.background_color, self.data, Button))
            .with_children(|parent| self.child.spawn(parent));
    }
}
