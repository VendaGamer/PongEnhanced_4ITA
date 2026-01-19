use std::fmt::Write;
use bevy::prelude::DerefMut;
use derive_more::{Deref, From, Into};
use serde::{Deserialize, Serialize};
use crate::components::ui::{UIOptionString};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize, Default, Debug)]
pub enum GameMode {
    #[default]
    Classic,
    UpsideDown,
    Modern,
    Blackout,
    Twisted,
}

impl UIOptionString for GameMode {
    fn push_ui_option_string(&self, string: &mut String) {
        let s = match self {
            GameMode::Classic => "Classic",
            GameMode::UpsideDown => "Upside Down",
            GameMode::Modern => "Modern",
            GameMode::Blackout => "Blackout",
            GameMode::Twisted => "Twisted",
        };

        string.push_str(s);
    }
}


#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq, Serialize,
         Deserialize, Default, From, Into, Deref, DerefMut)]
pub struct PlayerNum(pub u8);

impl UIOptionString for PlayerNum {
    fn push_ui_option_string(&self, string: &mut String) {
        write!(string, "{} Players", self.0).unwrap();
    }
}