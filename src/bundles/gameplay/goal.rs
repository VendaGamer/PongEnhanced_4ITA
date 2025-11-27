use crate::components::{Goal, Side};
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::{Bundle, Entity, Transform};

#[derive(Bundle)]
pub struct GoalBundle{
    pub goal: Goal,
    pub collider: Collider,
    pub transform: Transform,
    pub rigid_body: RigidBody
}

impl GoalBundle{
    pub fn new(team: Entity, side: Side) -> Self{
        let collider = Side::get_collider(side.clone());
        let transform = Side::get_transform(side.clone());

        Self{
            goal: Goal{
                team,
                side
            },
            collider,
            transform,
            rigid_body: RigidBody::Static
        }
    }
}