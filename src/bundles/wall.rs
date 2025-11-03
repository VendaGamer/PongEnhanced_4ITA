use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::Transform;
use crate::bundles::Bundle;
use crate::components::side::Side;
use crate::components::wall::Wall;

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub collider: Collider,
    pub transform: Transform,
    pub rigid_body: RigidBody
}

impl WallBundle {
    pub fn new(side: Side) -> Self {
        let collider = Side::get_collider(side);
        let transform = Side::get_transform(side);

        Self {
            wall: Wall {
                side
            },
            collider,
            transform,
            rigid_body: RigidBody::Static
        }
    }
}