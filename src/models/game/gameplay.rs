use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use crate::components::ui::{ChangeUIOptionString, UIOptionString};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
pub enum GameMode {
    #[default]
    Classic,
    UpsideDown,
    Modern,
    Blackout,
    Twisted,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize, Default, From, Into)]
pub struct PlayerNum(pub u8);

impl UIOptionString for PlayerNum{
    fn fill_ui_option_string(&self, string: &String) -> &str {
        string.
    }
}

impl ChangeUIOptionString for GameMode {
    fn to_ui_option_string(&self) -> &str {
        match self {
            GameMode::Classic => "Classic",
            GameMode::UpsideDown => "Upside Down",
            GameMode::Modern => "Modern",
            GameMode::Blackout => "Blackout",
            GameMode::Twisted => "Twisted",
        }.to_string()
    }
}