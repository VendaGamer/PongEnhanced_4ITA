use bevy::prelude::*;
use bevy::prelude::KeyCode::{KeyD, KeyS, KeyW};
use bevy::render::camera;
use bevy::text::FontFamily::Name;
use bevy::window::WindowEvent::WindowResized;

mod game;

// bring Paddle into scope
use game::paddle::Paddle;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            move_paddle
            )) // <-- don’t forget to register your system
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .run();
}

fn move_paddle(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>, // query paddle entities
) {
    if input.pressed(KeyW) {
        for mut transform in &mut query {
            transform.translation.y += 5.0; // move paddle up
        }
    }else if input.pressed(KeyS) {
        for mut transform in &mut query {
            transform.translation.y -= 5.0; // move paddle up
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {

    commands
        .spawn(Camera2d)
        .insert(Projection::from(OrthographicProjection {
            scaling_mode: camera::ScalingMode::Fixed { width: 1280.0, height: 720.0 },
            ..OrthographicProjection::default_2d()
        }));

    let white = materials.add(Color::WHITE);

    let paddle_width = 25.0;
    let paddle_height = 200.0;

    let paddle_mesh = meshes.add(Rectangle::new(paddle_width, paddle_height));

    // Spawn paddle
    commands.spawn((
        Mesh2d(paddle_mesh),
        MeshMaterial2d(white),
        Transform::from_translation(Vec3::new(640.0, 360.0, 0.0)),
        Paddle,
    ));
}
