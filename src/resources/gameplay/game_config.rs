use bevy::prelude::Resource;
use crate::components::area::Area;
use crate::components::{AreaShape, GameMode};

#[derive(Resource)]
pub struct GameModeConfig {
    pub current_mode: GameMode,
    pub initial_ball_speed: f32,
}

impl GameModeConfig {
    pub fn get_ball_speed(&self) -> f32 {
        match self.current_mode {
            GameMode::Classic => 400.0,
            GameMode::Modern => 600.0,
            GameMode::Twisted => 450.0,
            _ => 400.0,
        }
    }

    pub fn get_paddle_speed(&self) -> f32 {
        match self.current_mode {
            GameMode::Classic => 400.0,
            GameMode::UpsideDown => 300.0,
            GameMode::Modern => 500.0,
            _ => 400.0,
        }
    }
}