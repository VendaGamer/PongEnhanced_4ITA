use bevy::prelude::*;
use bevy::prelude::Projection::Orthographic;
use bevy::render::camera;
use crate::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (
            movement::move_ball,
            movement::move_paddle))
        .add_systems(Startup, setup);
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
        Projection::from(OrthographicProjection{
            scaling_mode: camera::ScalingMode::Fixed { width: 1280.0, height: 720.0 },
            ..default()
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
            linvel : Vect::new(-1000.0, 0.0),
            angvel : 0.4
        },
        Collider::ball(25.0),
        ColliderMassProperties::Density(5.0)
    ));
}