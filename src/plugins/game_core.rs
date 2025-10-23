use bevy_rapier2d::prelude::Velocity;
use crate::bundles::player::PlayerBundle;
use crate::bundles::*;
use crate::components::{Player, Team};
use crate::systems::*;

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
            (
                move_paddle,
                check_connection,
                handle_scoring
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

    let paddle_entity = commands.spawn(
        PaddleBundle::new(
            &mut meshes,
            &mut materials,
            Vec3::new(-600.0, 0.0, 0.0), // P1's position
        )
    ).id();

    let team1 = commands.spawn(Team{
        id : 0,
        name: "Bestie".into(),
        current_score: 0,
    }).id();

    let team2 = commands.spawn(Team{
        id: 1,
        name: "Chlapy".into(),
        current_score: 0,
    }).id();

    let team3 = commands.spawn(Team{
        id: 2,
        name: "Organgutani".into(),
        current_score: 0,
    }).id();

    let team4 = commands.spawn(Team{
        id: 3,
        name: "Prďky".into(),
        current_score: 0,
    }).id();

    commands.spawn(
        PlayerBundle::new(Player {
            id: 1,
            team: team1,
            name: "Player 1".into(),
        }));

    commands.spawn(
        PlayerBundle::new(Player {
            id: 2,
            team: team2,
            name: "Player 2".into(),
        }));

    commands.spawn(
        PlayerBundle::new(Player {
            id: 3,
            team: team3,
            name: "Player 3".into(),
        }));

    commands.spawn(
        PlayerBundle::new(Player {
            id: 4,
            team: team4,
            name: "Player 4".into(),
        }));

    commands.spawn(BallBundle::new(
        &mut meshes,
        &mut materials,
        Vec3::ZERO,
        Vec2::new(-300.0, 0.0)
    ));

}