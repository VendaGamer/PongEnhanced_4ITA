use crate::bundles::Entity;
use crate::components::{Paddle, Player};
use crate::resources::controls::PlayerAction;
use bevy::prelude::Bundle;
use leafwing_input_manager::input_map::InputMap;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub bindings: InputMap<PlayerAction>,
}

impl PlayerBundle {
    pub fn new(player: Player) -> Self{
        let bindings = Player::get_default_input_map(&player);
        PlayerBundle{
            player,
            bindings
        }
    }
}