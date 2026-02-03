use bevy::ecs::schedule::graph::Direction;
use bevy::math::CompassOctant;
use bevy::prelude::{Entity, Event};

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

#[derive(Event)]
pub struct UINavigated {
    pub direction: CompassOctant,
    pub entity: Entity,
}