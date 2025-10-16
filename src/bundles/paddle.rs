use std::sync::Arc;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::InputMap;
use crate::components::*;
use crate::Paddle;
use crate::resources::controls::Controls;
use crate::resources::PlayerControls;

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub mass: ColliderMassProperties,
}

impl PaddleBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        position: Vec3,
        controls: InputMap<Controls>,
    ) -> Self {
        Self {
            paddle: Paddle{
                player_controls: controls
            },
            mesh: Mesh2d(meshes.add(Rectangle::new(25.0, 200.0))),
            material: MeshMaterial2d(materials.add(Color::WHITE)),
            transform: Transform::from_translation(position),
            rigid_body: RigidBody::KinematicPositionBased,
            collider: Collider::cuboid(12.5, 100.0),
            mass: ColliderMassProperties::Density(100.0)
        }
    }
}