use crate::bundles::{children, default, BackgroundColor, BorderRadius, Bundle, ChildOf, Color, Node, Spawn, Text, TextColor, TextFont, UiRect, Val};
use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::prelude::*;
use bevy::text::FontSmoothing;


const GAME_TITLE: &'static str = "PONG ENHANCED";

#[derive(Bundle)]
pub struct LabelBundle {
    pub(crate) text: Text,
    pub(crate) font: TextFont,
    pub(crate) color: TextColor,
}
pub type MainMenuLabel = (Node, SpawnRelatedBundle<ChildOf, Spawn<LabelBundle>>);
pub type StatusLabel = (Node, BackgroundColor, BorderRadius, SpawnRelatedBundle<ChildOf, (Spawn<Text>, Spawn<TextFont>, Spawn<TextColor>)>);

impl LabelBundle {
    pub fn button_label(text: &str) -> Self{
        Self{
            text: Text::new(text),
            font: TextFont {
                font_size: 32.0,
                font_smoothing: FontSmoothing::None,
                ..default()
            },
            color: TextColor(Color::WHITE),
        }
    }

    pub fn game_title() -> MainMenuLabel {
        (
            Node{
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            },
            children![
                Self{
                    text: Text::new(GAME_TITLE),
                    font: TextFont {
                        font_size: 72.0,
                        font_smoothing: FontSmoothing::None,
                        ..default()
                    },
                    color: TextColor(Color::srgb(0.9, 0.9, 1.0)),
                }]
        )
    }
}