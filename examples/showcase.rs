use bevy::{color::palettes, prelude::*};
use bevy_ui_bits::*;

const HEART_TEXT_ID: usize = 1;
const HEART_BUTTON_ID: usize = 1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_ui)
        .add_systems(Update, handle_input)
        .run();
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let font = &Handle::default();

    let root = Root::new();

    let top = Container::height(Val::Percent(30.0));
    let middle = Container::size(Val::Percent(100.0), Val::Percent(40.0))
        .row()
        .justify_between();
    let left = Container::size(Val::Percent(50.0), Val::Percent(100.0))
        .background_color(palettes::css::DARK_BLUE.into())
        .justify_around();
    let right = Container::size(Val::Percent(50.0), Val::Percent(100.0))
        .background_color(palettes::css::DARK_RED.into())
        .justify_around();
    let bottom = Container::size(Val::Percent(100.0), Val::Percent(30.0))
        .background_color(palettes::css::GOLDENROD.into());

    let title =
        EmbossedText::large("bevy_ui_bits has", font).color(palettes::css::GOLDENROD.into());
    let simple_text = SimpleText::medium("Simple text", font);
    let embossed_text = EmbossedText::medium("Embossed text", font);
    let dynamic_text = DynamicTextBuilder::medium("Dynamic text: ", font)
        .id(HEART_TEXT_ID)
        .initial_dynamic_text("0");
    let rectangle_button = UiButton::rectangle().background_color(palettes::css::GOLDENROD.into());
    let rectangle_button_text = EmbossedText::medium("Buttons", font);
    let square_button = UiButton::square()
        .id(HEART_BUTTON_ID)
        .background_color(palettes::css::MIDNIGHT_BLUE.into());
    let square_button_text = SimpleText::medium("+1", font);
    let containers = SimpleText::medium("and containers!", font).color(palettes::css::BLACK.into());

    commands.spawn((
        root,
        children![
            (top, children![title]),
            (
                middle,
                children![
                    (
                        left,
                        children![simple_text, embossed_text, dynamic_text.build()]
                    ),
                    (
                        right,
                        children![
                            (rectangle_button, children![rectangle_button_text]),
                            (square_button, children![square_button_text])
                        ]
                    )
                ]
            ),
            (bottom, children![containers])
        ],
    ));
}

fn handle_input(
    mut writer: TextUiWriter,
    mut query: Query<
        (&UiButtonData, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut texts: Query<(Entity, &DynamicTextData)>,
) {
    for (button_data, interaction, mut background_color) in &mut query {
        if button_data.id == HEART_BUTTON_ID {
            match *interaction {
                Interaction::None => {
                    *background_color = palettes::css::MIDNIGHT_BLUE.into();
                }
                Interaction::Hovered => {
                    *background_color = palettes::css::TEAL.into();
                }
                Interaction::Pressed => {
                    *background_color = palettes::css::AQUA.into();
                    for (entity, data) in texts.iter_mut() {
                        if matches!(data.id, HEART_TEXT_ID) {
                            if let Some(mut text) = writer.get_text(entity, 1) {
                                *text =
                                    format!("{}", text.as_str().parse::<usize>().unwrap_or(0) + 1);
                            }
                        }
                    }
                }
            }
        }
    }
}
