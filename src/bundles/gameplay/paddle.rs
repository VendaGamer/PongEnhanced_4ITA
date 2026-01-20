use crate::Paddle;
use avian2d::prelude::*;
use bevy::prelude::*;
use crate::models::game::area::PlayerID;

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
        size: Vec2,
        goal: Entity,
        id: PlayerID
    ) -> Self {
        Self {
            paddle: Paddle {
                goal,
                id
            },
            mesh: Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
            material: MeshMaterial2d(materials.add(Color::WHITE)),
            transform: Transform::from_translation(position),
            rigid_body: RigidBody::Kinematic,
            collider: Collider::rectangle(size.x, size.y),
            restitution: Restitution::new(0.0),
        }
    }
}