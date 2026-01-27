use bevy::prelude::{Entity, Event};
use crate::bundles::EntityEvent;

#[derive(EntityEvent)]
pub struct GoalScored(pub Entity);

#[derive(Event)]
pub struct BallBounced {
    pub paddle: Entity,
    pub ball: Entity,
}