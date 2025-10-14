use bevy::prelude::{Handle, Resource};
use crate::resources::PlayerControls;

#[derive(Resource)]
pub struct Controls {
    pub player1: Handle<PlayerControls>,
    pub player2: Handle<PlayerControls>,
    pub player3: PlayerControls,
    pub player4: PlayerControls
}