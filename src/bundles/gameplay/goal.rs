use crate::components::{Goal};
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::{Bundle, Entity, Transform};
use crate::models::game::area::AreaSide;

#[derive(Bundle)]
pub struct GoalBundle {
    pub goal: Goal,
    pub collider: Collider,
    pub transform: Transform,
    pub rigid_body: RigidBody
}

impl GoalBundle{
    pub fn new(team: Entity, side: AreaSide) -> Self{
        let collider = AreaSide::get_collider(side.clone());
        let transform = AreaSide::get_transform(side.clone());

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