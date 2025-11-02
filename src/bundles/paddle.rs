use crate::Paddle;
use bevy::prelude::*;
use avian2d::prelude::*;

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub restitution: Restitution,
}

impl PaddleBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        position: Vec3,
    ) -> Self {
        Self {
            paddle: Paddle,
            mesh: Mesh2d(meshes.add(Rectangle::new(25.0, 200.0))),
            material: MeshMaterial2d(materials.add(Color::WHITE)),
            transform: Transform::from_translation(position),
            rigid_body: RigidBody::Kinematic,
            collider: Collider::rectangle(25.0, 200.0),
            restitution: Restitution::new(1.0),
        }
    }
}