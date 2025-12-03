use avian2d::prelude::{Collider, Restitution, RigidBody};
use bevy::prelude::Transform;
use crate::bundles::Bundle;
use crate::components::*;
use crate::models::game::area::AreaSide;

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub collider: Collider,
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub restitution: Restitution
}

impl WallBundle {
    pub fn new(side: AreaSide) -> Self {
        let collider = AreaSide::get_collider(side);
        let transform = AreaSide::get_transform(side);

        Self {
            wall: Wall {
                side
            },
            collider,
            transform,
            rigid_body: RigidBody::Static,
            restitution: Restitution::new(0.0)
        }
    }
}