use crate::bundles::Entity;
use crate::components::{ControlledPaddle, Player};
use crate::resources::controls::PlayerAction;
use bevy::prelude::Bundle;
use leafwing_input_manager::input_map::InputMap;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub paddle: ControlledPaddle,
    pub bindings: InputMap<PlayerAction>,

}

impl PlayerBundle {
    pub fn new(player: Player, paddle: Entity) -> Self{
        let bindings = Player::get_default_input_map(&player);

        PlayerBundle{
            player,
            paddle: ControlledPaddle::new(paddle),
            bindings
        }
    }
}