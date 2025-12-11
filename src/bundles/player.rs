use crate::components::Player;
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
}

impl PlayerBundle {
    pub fn new(id: u8) -> Self {
        Self {
            player: Player {
                id,
                bindings: Player::get_default_input_map(id),
            }
        }
    }
}