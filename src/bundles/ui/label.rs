use crate::bundles::{default, Bundle, Color, Text, TextColor, TextFont};
const TITLE: &str = "PONG ENHANCED";

#[derive(Bundle)]
struct Label {
    text: Text,
    font: TextFont,
    color: TextColor,
}

impl Label {
    pub fn button(text: &str) -> Self{
        Self{
            text: Text::new(text),
            font: TextFont {
                font_size: 32.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
        }
    }

    pub fn game_title() -> Self{
        Self{
            text: Text::new(TITLE),
            font: TextFont {
                font_size: 72.0,
                ..default()
            },
            color: TextColor(Color::srgb(0.9, 0.9, 1.0)),
        }
    }
}