use crate::Paddle;
use bevy::prelude::*;
use avian2d::prelude::*;
use crate::components::Player;

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

    pub fn spawn_team(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        position: Vec3,
        size: Vec2,
        goal: Entity
    ){

        commands.spawn_batch()

    }

    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        position: Vec3,
        size: Vec2,
        goal: Entity
    ) -> Self {
        Self {
            paddle: Paddle{
                goal
            },
            mesh: Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
            material: MeshMaterial2d(materials.add(Color::WHITE)),
            transform: Transform::from_translation(position),
            rigid_body: RigidBody::Kinematic,
            collider: Collider::rectangle(size.x, size.y),
            restitution: Restitution::new(1.0),
        }
    }
}