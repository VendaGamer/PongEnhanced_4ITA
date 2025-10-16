use bevy::input::keyboard::Key::Control;
use crate::bundles::*;
use crate::resources::controls::Controls;
use crate::systems::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
            (
                move_paddle,
            ))
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    
    commands.spawn(CameraBundle::default());
    commands.spawn(
        PaddleBundle::new(
            &mut meshes,
            &mut materials,
            Vec3::new(-600.0, 0.0, 0.0),
            InputMap::new([
                (Controls::Up, KeyCode::KeyW),
                (Controls::Down, KeyCode::KeyS),
                (Controls::Left, KeyCode::KeyA),
                (Controls::Right, KeyCode::KeyD),
                (Controls::Dash, KeyCode::ShiftLeft),
                (Controls::Push, KeyCode::Space),
            ])
        ));

    commands.spawn(BallBundle::new(
        &mut meshes,
        &mut materials,
        Vec3::ZERO,
        Vec2::new(-100.0, 0.0)
    ));
}