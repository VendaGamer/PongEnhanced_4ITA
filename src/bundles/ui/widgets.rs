use crate::bundles::{default, Bundle, Color, Node, Text, TextColor, TextFont, UiRect, Val};
use bevy::prelude::*;
use bevy::text::FontSmoothing;

const GAME_TITLE: &'static str = "PONG ENHANCED";

#[derive(Bundle)]
pub struct LabelBundle;

impl LabelBundle {
    pub fn button_label(text: impl Into<String>) -> impl Bundle {
        (
            Text::new(text),
            TextFont {
                font_size: 32.0,
                font_smoothing: FontSmoothing::None,
                ..default()
            },
            TextColor(Color::WHITE),
        )
    }

    pub fn game_title() -> impl Bundle {
        (
            Node {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            },
            Children::spawn_one((
                Text::new(GAME_TITLE),
                TextFont {
                    font_size: 72.0,
                    font_smoothing: FontSmoothing::None,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
            )),
        )
    }

    pub fn custom(text: &str, color: Color, size: f32) -> impl Bundle {
        (
            Text::new(text),
            TextFont {
                font_size: size,
                font_smoothing: FontSmoothing::None,
                ..default()
            },
            TextColor(color),
        )
    }
}
