use crate::components::Player;
use crate::models::game::area::LocalPlayerID;
use crate::resources::PlayerAction;
use bevy::prelude::{Bundle, Entity};
use leafwing_input_manager::input_map::InputMap;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub bindings: InputMap<PlayerAction>,
}

impl PlayerBundle {
    pub fn new_keyboard(id: u8) -> Self {
        Self {
            player: Player {
                id: LocalPlayerID::KeyboardPlayer(id)
            },
            bindings: Player::get_default_input_map(id),
        }
    }

    pub fn new_gamepad(gamepad: Entity) -> Self {
        Self {
            player: Player {
                id: LocalPlayerID::Gamepad(gamepad)
            },
            bindings: Player::get_gamepad_input_map(gamepad),
        }
    }
}