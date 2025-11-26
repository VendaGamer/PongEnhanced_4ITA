use bevy::ecs::spawn::SpawnRelatedBundle;
use crate::bundles::*;
use crate::bundles::ui;

const TITLE: &str = "PONG ENHANCED";

#[derive(Bundle)]
pub struct LabelBundle {
    text: Text,
    font: TextFont,
    color: TextColor,
}
pub type MainMenuLabel = (Node, SpawnRelatedBundle<ChildOf, Spawn<LabelBundle>>);

impl LabelBundle {
    pub fn button_label(text: &str) -> Self{
        Self{
            text: Text::new(text),
            font: TextFont {
                font_size: 32.0,
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
                    text: Text::new(TITLE),
                    font: TextFont {
                        font_size: 72.0,
                        ..default()
                    },
                    color: TextColor(Color::srgb(0.9, 0.9, 1.0)),
                }]
        )
    }
}