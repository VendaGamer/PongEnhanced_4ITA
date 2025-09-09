mod game;

use bevy::prelude::*;
use bevy::prelude::KeyCode::*;
use bevy::render::camera;
use game::paddle::Paddle;


fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, move_paddle)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .run();
}

fn move_paddle(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
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

    commands.spawn((
        Mesh2d(paddle_mesh),
        MeshMaterial2d(white),
        Transform::from_translation(Vec3::new(0.0, 360.0, 0.0)),
        Paddle,
    ));
}
