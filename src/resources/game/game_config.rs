use crate::components::ui::{IntoUIOptionString, OptionSelector};
use crate::models::game::area::AreaShape;
use crate::models::game::gameplay::GameMode;
use bevy::prelude::{Res, Resource, UVec2};
use bevy::window::{MonitorSelection, WindowMode};
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};

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
    pub fn get_current_monitor_or_first(&self) -> Option<&MonitorInfo> {
        let index = self.selected_monitor.unwrap_or_default();

        self.monitors.get(index)
    }
}


#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub monitor_selection: MonitorSelection,
    pub name: String,
    pub refresh_rates: Vec<RefreshRate>,
    pub resolutions: Vec<Resolution>,
    pub bit_depths: Vec<BitDepth>,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, From, Into)]
pub struct RefreshRate(pub u32);

impl IntoUIOptionString for RefreshRate {
    fn as_ui_option_string(&self) -> String {
        format!("{}Hz", self.0 / 1000)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, From, Into)]
pub struct Resolution(pub UVec2);

impl IntoUIOptionString for Resolution {
    fn as_ui_option_string(&self) -> String {
        format!("{} x {}", self.0.x, self.0.y)
    }
}
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, From, Into)]
pub struct BitDepth(pub u16);

impl IntoUIOptionString for BitDepth {
    fn as_ui_option_string(&self) -> String {
        format!("{}-bit", self.0)
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