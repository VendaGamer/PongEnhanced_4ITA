use crate::components::Goal;
use crate::models::game::area::TeamInfo;
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::{Bundle, Transform};

#[derive(Bundle)]
pub struct GoalBundle {
    pub goal: Goal,
    pub collider: Collider,
    pub transform: Transform,
    pub rigid_body: RigidBody,
}

impl GoalBundle {
    pub fn new(team: &TeamInfo) -> Self {
        Self {
            goal: Goal {
                side: team.area_side,
            },
            collider: team.area_side.get_collider(),
            transform: team.area_side.get_transform(),
            rigid_body: RigidBody::Static,
        }
    }
}
