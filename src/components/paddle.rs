use bevy::prelude::Component;
use crate::resources::PlayerControls;

#[derive(Component)]
pub struct Paddle{
    pub player_controls: PlayerControls
}
