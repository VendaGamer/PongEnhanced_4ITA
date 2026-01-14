use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum GameMode {
    Classic,
    UpsideDown,
    Modern,
    Blackout,
    Twisted,
}