use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2d);

    let circle = meshes.add(Circle::new(50.0)); // radius 50

    let white = materials.add(Color::WHITE);


    commands.spawn((
        Mesh2d(circle),
        MeshMaterial2d(white),
        Transform::from_translation(Vec3::ZERO),
    ));
}