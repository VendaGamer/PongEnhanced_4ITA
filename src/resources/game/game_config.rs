use crate::models::game::area::AreaShape;
use crate::models::game::gameplay::GameMode;
use crate::models::game::settings::ScreenMode;
use bevy::prelude::{Resource, UVec2};
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Copy, Clone)]
pub struct GameSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub screen_mode: ScreenMode,
    pub resolution: UVec2,
    pub vsync: bool,
    pub show_fps: bool,
}


impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 50.0,
            sfx_volume: 50.0,
            screen_mode: ScreenMode::BorderlessFullScreen,
            resolution: UVec2::new(1280, 720),
            vsync: true,
            show_fps: true,
        }
    }
}

#[derive(Resource, Hash, PartialEq, Eq)]
pub struct GameModeConfig {
    pub game_mode: GameMode,
    pub area_shape: AreaShape,
    pub win_score: u32,
}

impl Default for GameModeConfig {
    fn default() -> Self {
        Self{
            game_mode: GameMode::Classic,
            area_shape: AreaShape::TwoSide(None),
            win_score: 10,
        }
    }
}

impl GameModeConfig {

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