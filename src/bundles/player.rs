use crate::components::Player;
use crate::models::game::area::LocalPlayerID;
use crate::resources::PlayerAction;
use bevy::prelude::{Bundle, Entity};
use leafwing_input_manager::input_map::InputMap;
use lightyear::prelude::{NetworkTarget, Replicate};
use crate::models::game::gameplay::PlayerId;
use crate::networking::protocol::RemotePlayerId;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub bindings: InputMap<PlayerAction>,
}

impl PlayerBundle {
    pub fn new(id: LocalPlayerID) -> Self {
        Self {
            player: Player {
                id: PlayerId::Local(id)
            },
            bindings: Player::get_input_map(id),
        }
    }

    pub fn new_network(id: RemotePlayerId) -> impl Bundle {
        (
            Replicate::to_clients(NetworkTarget::All),
            PlayerBundle::new(id.1)
        )
    }
}