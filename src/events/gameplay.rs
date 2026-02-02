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
