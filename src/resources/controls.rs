use std::sync::Arc;
use bevy::prelude::{Handle, Resource};
use crate::resources::PlayerControls;

#[derive(Resource)]
pub struct Controls {
    pub player1: Arc<PlayerControls>,
    pub player2: Arc<PlayerControls>,
    pub player3: Arc<PlayerControls>,
    pub player4: Arc<PlayerControls>
}