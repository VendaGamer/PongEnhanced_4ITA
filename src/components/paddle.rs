use bevy::prelude::{Component, Handle};
use leafwing_input_manager::prelude::InputMap;
use crate::resources::controls::Controls;
use crate::resources::PlayerControls;

#[derive(Component)]
pub struct Paddle{
    pub player_controls: Handle<InputMap<Controls>>
}
