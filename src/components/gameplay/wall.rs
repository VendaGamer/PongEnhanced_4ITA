use bevy::prelude::Component;
use crate::components::side::Side;

#[derive(Component)]
pub struct Wall {
    pub side: Side,
}