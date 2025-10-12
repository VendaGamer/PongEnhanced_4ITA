use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy_rapier2d::dynamics::{RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, ColliderMassProperties};
use bevy_rapier2d::prelude::*;
use crate::Ball;

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub collider: Collider,
    pub restitution: Restitution,
    pub mass: ColliderMassProperties,
    pub active_events: ActiveEvents,
    pub gravity_scale: GravityScale,
    pub ccd: Ccd,
}

impl BallBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        position: Vec3,
        initial_velocity: Vec2,
    ) -> Self {
        Self {
            ball: Ball,
            mesh: Mesh2d(meshes.add(Circle::new(25.0))),
            material: MeshMaterial2d(materials.add(Color::WHITE)),
            transform: Transform::from_translation(position),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity {
                linvel: initial_velocity,
                angvel: 0.0,
            },
            collider: Collider::ball(25.0),
            restitution: Restitution{
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Max
            }, // Perfect bounce
            mass: ColliderMassProperties::Density(5.0),
            active_events: ActiveEvents::COLLISION_EVENTS,
            gravity_scale: GravityScale(0.0), // Disable gravity
            ccd: Ccd::enabled(),
        }
    }
}