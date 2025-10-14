use bevy::prelude::{Asset, KeyCode, TypePath};

#[derive(Asset, TypePath)]
pub struct PlayerControls{
    pub up: KeyCode,
    pub down: KeyCode,
    pub dash: KeyCode,
    pub push: KeyCode,
}