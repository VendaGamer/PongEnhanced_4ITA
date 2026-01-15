use std::sync::{Arc, Weak};
use crate::models::game::area::AreaShape;
use crate::models::game::gameplay::GameMode;
use bevy::prelude::{Res, Resource, UVec2};
use bevy::window::{MonitorSelection, WindowMode};
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use crate::components::ui::UIOptionString;

#[derive(Resource, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub video_mode: WindowMode,
    pub vsync: bool,
    pub show_fps: bool,
}

#[derive(Resource, Clone, Eq, PartialEq, Debug)]
pub struct PendingSettings {
    pub video_mode: WindowMode,
    pub vsync: bool,
    pub show_fps: bool,
}

impl From<&Res<'_, GameSettings>> for PendingSettings {
    fn from(settings: &Res<'_, GameSettings>) -> Self {
        Self{
            video_mode: settings.video_mode,
            vsync: settings.vsync,
            show_fps: settings.show_fps,
        }
    }
}



#[derive(Resource, Clone, Default, Debug)]
pub struct Monitors {
    pub monitors: Vec<MonitorInfo>,
    pub selected_monitor: Option<usize>,
}

impl Monitors {
    pub fn get_current_monitor(&self) -> Option<&MonitorInfo> {
        let index = self.selected_monitor.unwrap_or_default();
        self.monitors.get(index)
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MonitorInfo {
    pub monitor_selection: MonitorSelection,
    pub name: String,
    pub refresh_rates: Arc<Vec<Box<RefreshRate>>>,
    pub resolutions: Arc<Vec<Box<Resolution>>>,
    pub bit_depths: Arc<Vec<Box<BitDepth>>>,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, From, Into)]
pub struct RefreshRate(pub u32);

impl UIOptionString for RefreshRate {

    fn fill_ui_option_string(&self, string: &mut String) {
        string.push_str(format!("{} Hz", self.0).as_str());
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, From, Into)]
pub struct Resolution(pub UVec2);

impl UIOptionString for Resolution {
    fn fill_ui_option_string(&self, string: &mut String) {
        string.push_str(format!("{} x {}", self.0.x, self.0.y).as_str());
    }
}
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, From, Into)]
pub struct BitDepth(pub u16);

impl UIOptionString for BitDepth {
    fn fill_ui_option_string(&self, string: &mut String) {
        string.push_str(format!("{}-bit", self.0).as_str())
    }
}


impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 50.0,
            sfx_volume: 50.0,
            video_mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
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
            area_shape: AreaShape::TwoSide(
                [
                    // Team 1
                    crate::models::game::area::TeamInfo {
                        name: "Team 1".to_string(),
                        current_score: 0,
                        area_side: crate::models::game::area::AreaSide::Left,
                        players: vec![],
                    },
                    // Team 2
                    crate::models::game::area::TeamInfo {
                        name: "Team 2".to_string(),
                        current_score: 0,
                        area_side: crate::models::game::area::AreaSide::Right,
                        players: vec![],
                    },
                ]
            ),
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