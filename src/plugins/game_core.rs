use crate::bundles::*;
use crate::resources::controls::Controls;
use crate::resources::PlayerControls;
use crate::systems::*;
use bevy::prelude::*;
use std::sync::Arc;

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
            (
                move_paddle,
            ))
            .insert_resource(
                Controls{
                    player1: Arc::new(PlayerControls{
                        up: KeyCode::KeyW,
                        down: KeyCode::KeyS,
                        dash: KeyCode::ShiftLeft,
                        push: KeyCode::Space
                    }),
                    player2: Arc::new(PlayerControls {
                        up: KeyCode::ArrowUp,
                        down: KeyCode::ArrowDown,
                        dash: KeyCode::ShiftRight,
                        push: KeyCode::ControlRight
                    }),
                    player3: Arc::new(PlayerControls {
                        up: KeyCode::KeyI,
                        down: KeyCode::KeyK,
                        dash: KeyCode::KeyJ,
                        push: KeyCode::KeyL
                    }),
                    player4: Arc::new(PlayerControls {
                        up: KeyCode::Numpad8,
                        down: KeyCode::Numpad5,
                        dash: KeyCode::Numpad9,
                        push: KeyCode::Numpad0
                    }),
                }
            )
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    controls: Res<Controls>
) {
    
    commands.spawn(CameraBundle::default());
    commands.spawn((
        PaddleBundle::new(
            &mut meshes,
            &mut materials,
            Vec3::new(-600.0, 0.0, 0.0),
        ),
    ));

    commands.spawn(BallBundle::new(
        &mut meshes,
        &mut materials,
        Vec3::ZERO,
        Vec2::new(-100.0, 0.0),
    ));
}