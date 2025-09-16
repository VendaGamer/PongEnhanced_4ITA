mod game;

use bevy::core_pipeline::bloom::{Bloom, BloomCompositeMode};
use bevy::diagnostic::*;
use bevy::ecs::schedule::*;
use bevy::prelude::KeyCode::*;
use bevy::prelude::*;
use bevy::render::*;
use bevy_rapier2d::prelude::*;
use game::ball::Ball;
use game::paddle::Paddle;
use game::player::Player;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, (
                move_paddle,
            ))
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierDebugRenderPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .edit_schedule(Update, |schedule| {
            schedule.set_executor_kind(ExecutorKind::MultiThreaded);
        })
        .run();
}

fn move_paddle(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    let move_amount = time.delta_secs() * 200.0;

    if input.pressed(KeyW) {
        for mut transform in &mut query {
            transform.translation.y += move_amount;
        }
    }else if input.pressed(KeyS) {
        for mut transform in &mut query {
            transform.translation.y -= move_amount;
        }
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
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Projection::from(OrthographicProjection {
            scaling_mode: camera::ScalingMode::Fixed { width: 1280.0, height: 720.0 },
            ..OrthographicProjection::default_2d()
        }),
        Bloom{
            composite_mode : BloomCompositeMode::Additive,
            ..default()
        }
    ));

    let white = materials.add(Color::WHITE);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(25.0, 200.0))),
        MeshMaterial2d(white.clone()),
        Transform::from_translation(Vec3::new(-600.0, 0.0, 0.0)),
        Paddle,
        Player,
        RigidBody::Dynamic,
        Collider::cuboid(25.0, 200.0)
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(white),
        Transform::from_translation(Vec3::ZERO),
        Ball,
        RigidBody::Dynamic,
        Collider::ball(25.0),
    ));
}
