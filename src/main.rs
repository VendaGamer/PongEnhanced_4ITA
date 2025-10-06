mod components;
mod resources;

use bevy::prelude::*;
use bevy::render::*;
use bevy_rapier2d::prelude::*;
use crate::components::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, (
                move_ball,
                apply_forces
        ))
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)
        ))
        .run();
}

fn move_ball(
    mut query: Query<&mut Velocity, With<Ball>>
){
    for mut ball in &mut query {
        ball.linvel += Vec2::new(-0.1, 0.0);
    }
}

fn apply_forces(
    mut ext_forces: Query<&mut ExternalForce>,
    mut ext_impulses: Query<&mut ExternalImpulse>,
) {
    // Apply forces.
    for mut ext_force in ext_forces.iter_mut() {
        ext_force.force = Vec2::new(1000.0, 2000.0);
        ext_force.torque = 0.4;
    }

    // Apply impulses.
    for mut ext_impulse in ext_impulses.iter_mut() {
        ext_impulse.impulse = Vec2::new(100.0, 200.0);
        ext_impulse.torque_impulse = 0.4;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

commands.spawn((
        Camera2d,
        Camera{
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Projection::from(OrthographicProjection {
            scaling_mode: camera::ScalingMode::Fixed { width: 1280.0, height: 720.0 },
            ..OrthographicProjection::default_2d()
        })
    ));

    let white = materials.add(Color::WHITE);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(25.0, 200.0))),
        MeshMaterial2d(white.clone()),
        Transform::from_translation(Vec3::new(-600.0, 0.0, 0.0)),
        Paddle,
        Player,
        RigidBody::KinematicPositionBased,
        Collider::cuboid(25.0, 200.0),
        ColliderMassProperties::Density(100.0)
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(white),
        Transform::from_translation(Vec3::ZERO),
        Ball,
        RigidBody::KinematicVelocityBased,
        Velocity{
            linvel : Vec2::new(-1000.0, 0.0),
            angvel : 0.4
        },
        Collider::ball(25.0),
        ColliderMassProperties::Density(5.0)
    ));
}
