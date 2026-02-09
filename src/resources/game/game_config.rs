use crate::components::ui::UIOptionString;
use crate::models::game::area::AreaShape;
use crate::models::game::gameplay::GameMode;
use bevy::prelude::{Deref, Resource, UVec2};
use bevy::window::{MonitorSelection, PresentMode, VideoMode, WindowMode};
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Resource, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub window_mode: WindowMode,
    pub window_resolution: Option<UVec2>,
    pub vsync: PresentMode,
}

#[derive(Resource, Clone, Eq, PartialEq, Debug)]
pub struct PendingSettings {
    pub window_mode: WindowMode,
    pub window_resolution: Option<UVec2>,
    pub vsync: PresentMode,
}

impl From<&GameSettings> for PendingSettings {
    fn from(settings: &GameSettings) -> Self {
        Self {
            window_mode: settings.window_mode,
            window_resolution: settings.window_resolution,
            vsync: settings.vsync,
        }
    }
}

#[derive(Resource, Debug)]
pub struct Monitors {
    pub monitors: Arc<Vec<MonitorInfo>>,
    pub selected_monitor: usize,
}

#[derive(Resource, Debug, Default)]
pub struct OnlineGameConfig {
    pub server_name: String,
    pub pass: Option<String>,
}

impl UIOptionString for MonitorInfo {
    fn push_ui_option_string(&self, string: &mut String) {
        string.push_str(&*self.name);
    }
}

impl Monitors {
    pub fn get_current_monitor(&self) -> &MonitorInfo {
        self.monitors
            .get(self.selected_monitor)
            .expect("no monitor found")
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MonitorInfo {
    pub monitor_selection: MonitorSelection,
    pub native_mode: VideoMode,
    pub name: String,
    pub refresh_rates: Arc<Vec<RefreshRate>>,
    pub resolutions: Arc<Vec<Resolution>>,
    pub bit_depths: Arc<Vec<BitDepth>>,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, From, Into)]
pub struct RefreshRate(pub u32);

impl UIOptionString for RefreshRate {
    fn push_ui_option_string(&self, string: &mut String) {
        string.push_str(format!("{} Hz", self.0 / 1000).as_str());
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, From, Into, Deref)]
pub struct Resolution(pub UVec2);

impl UIOptionString for Resolution {
    fn push_ui_option_string(&self, string: &mut String) {
        string.push_str(format!("{} x {}", self.0.x, self.0.y).as_str());
    }
}
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, From, Into)]
pub struct BitDepth(pub u16);

impl UIOptionString for BitDepth {
    fn push_ui_option_string(&self, string: &mut String) {
        string.push_str(format!("{}-bit", self.0).as_str())
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 50.0,
            sfx_volume: 50.0,
            window_mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            vsync: PresentMode::AutoVsync,
            window_resolution: None,
        }
    }
}

#[derive(Resource, Hash, PartialEq, Eq, Debug)]
pub struct GameModeConfig {
    pub game_mode: GameMode,
    pub area_shape: AreaShape,
    pub win_score: u32,
}

impl Default for GameModeConfig {
    fn default() -> Self {
        Self {
            game_mode: GameMode::Classic,
            area_shape: AreaShape::default(),
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
