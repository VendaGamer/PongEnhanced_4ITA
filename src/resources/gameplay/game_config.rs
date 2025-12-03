use crate::components::{GameMode};
use bevy::prelude::Resource;
use crate::models::game::area::AreaShape;

#[derive(Resource, Hash, PartialEq, Eq)]
pub struct GameConfig {
    pub game_mode: GameMode,
    pub area_shape: AreaShape,
}

impl GameConfig {
    pub fn get_ball_speed(&self) -> f32 {
        match self.game_mode {
            GameMode::Classic => 400.0,
            GameMode::Modern => 600.0,
            GameMode::Twisted => 450.0,
            _ => 400.0,
        }
    }

    pub fn get_paddle_speed(&self) -> f32 {
        match self.game_mode {
            GameMode::Classic => 400.0,
            GameMode::UpsideDown => 300.0,
            GameMode::Modern => 500.0,
            _ => 400.0,
        }
    }
}