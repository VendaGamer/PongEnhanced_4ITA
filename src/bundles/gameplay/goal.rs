use crate::components::{Goal};
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::{Bundle, Entity, Transform};
use crate::models::game::area::{AreaSide, Team};

#[derive(Bundle)]
pub struct GoalBundle {
    pub goal: Goal,
    pub collider: Collider,
    pub transform: Transform,
    pub rigid_body: RigidBody
}

impl GoalBundle{
    pub fn new(team: &Team) -> Self{

        Self{
            goal: Goal{
                side: team.area_side,
            },
            collider: team.area_side.get_collider(),
            transform: team.area_side.get_transform(),
            rigid_body: RigidBody::Static
        }
    }
}