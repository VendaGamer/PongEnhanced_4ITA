use crate::utils::screen::ZERO_DAMPING;
use crate::Ball;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::*;
use bevy::prelude::*;
use avian2d::prelude::*;

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub linear_velocity: LinearVelocity,
    pub angular_velocity: AngularVelocity,
    pub collider: Collider,
    pub friction: Friction,
    pub restitution: Restitution,
    pub damping: LinearDamping,
    pub collision_layers: CollisionLayers,
    pub gravity_scale: GravityScale,
    pub ccd: SweptCcd,
}

impl BallBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        position: Vec3,
        initial_velocity: Vec2,
        radius: f32,
    ) -> Self {
        Self {
            ball: Ball{
                initial_velocity
            },
            mesh: Mesh2d(meshes.add(Circle::new(radius))),
            material: MeshMaterial2d(materials.add(Color::WHITE)),
            transform: Transform::from_translation(position),
            rigid_body: RigidBody::Dynamic,
            linear_velocity: LinearVelocity(initial_velocity),
            angular_velocity: AngularVelocity(0.0),
            collider: Collider::circle(radius),
            restitution: Restitution::new(1.0),
            friction: Friction::new(0.0),
            damping: ZERO_DAMPING,
            collision_layers: CollisionLayers::default(),
            gravity_scale: GravityScale(0.0),
            ccd: SweptCcd::default(),
        }
    }
}