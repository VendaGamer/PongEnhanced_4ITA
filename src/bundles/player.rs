use crate::components::Player;
use bevy::prelude::Bundle;
use leafwing_input_manager::input_map::InputMap;
use crate::resources::PlayerAction;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub bindings: InputMap<PlayerAction>,
}

impl PlayerBundle {
    pub fn new(id: u8) -> Self {
        Self {
            player: Player {
                id
            },
            bindings: Player::get_default_input_map(id),
        }
    }
}