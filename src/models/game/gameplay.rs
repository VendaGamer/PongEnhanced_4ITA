use crate::components::ui::UIOptionString;
use crate::models::game::area::LocalPlayerID;
use crate::networking::protocol::RemotePlayerId;
use bevy::prelude::DerefMut;
use derive_more::{Deref, From, Into};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize, Default, Debug)]
pub enum GameMode {
    #[default]
    Classic,
    UpsideDown,
    Modern,
    Blackout,
    Twisted,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PlayerId {
    Network(RemotePlayerId),
    Local(LocalPlayerID),
}

impl PlayerId {
    pub fn local(&self) -> LocalPlayerID {
        match self {
            PlayerId::Network(id) => id.1,
            PlayerId::Local(id) => *id,
        }
    }
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

#[derive(
    Clone,
    Debug,
    Copy,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
    Default,
    From,
    Into,
    Deref,
    DerefMut,
)]
pub struct PlayerNum(pub u8);

impl UIOptionString for PlayerNum {
    fn push_ui_option_string(&self, string: &mut String) {
        write!(string, "{} Players", self.0).unwrap();
    }
}
