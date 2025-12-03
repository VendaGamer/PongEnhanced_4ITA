use std::iter::Map;
use bevy::prelude::Resource;
use crate::components::area::Area;
use crate::components::{AreaShape, GameMode, Side, Team};

#[derive(Resource, Hash, PartialEq, Eq)]
pub struct GameConfig {
    pub game_mode: GameMode,
    pub area_shape: AreaShape,
    pub team: Map<Team, Side>
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