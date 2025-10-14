use bevy::prelude::Component;
use crate::components::PlayerControls;

#[derive(Component)]
pub struct Player{
    pub player_controls: PlayerControls
}