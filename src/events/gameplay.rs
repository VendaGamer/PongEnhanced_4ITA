use bevy::prelude::{Entity, Event};
use crate::bundles::EntityEvent;

#[derive(Event)]
pub struct GoalScored {
    pub ball: Entity,
    pub goal: Entity,
}

#[derive(Event)]
pub struct BallBounced {
    pub paddle: Entity,
    pub ball: Entity,
}