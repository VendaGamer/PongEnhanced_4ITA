use bevy::prelude::{Component, Entity};
use crate::components::side::Side;

#[derive(Component)]
pub struct Goal {
    pub team: Entity,
    pub side: Side
}