use avian2d::prelude::*;
use crate::bundles::player::PlayerBundle;
use crate::bundles::*;
use crate::components::*;
use crate::components::side::Side;
use crate::components::wall::Wall;
use crate::systems::*;
use crate::utils::FIXED_DIMENSIONS;
use crate::utils::screen::PADDLE_SIZE;

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
            (
                move_paddle,
                check_connection,
                handle_scoring,
                maintain_ball_speed,
                paddle_hit_dynamics,
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

    // === SPAWN TEAMS ===
    let team1 = commands.spawn(Team {
        id: 0,
        name: "Team Left".into(),
        current_score: 0,
    }).id();

    let team2 = commands.spawn(Team {
        id: 1,
        name: "Team Right".into(),
        current_score: 0,
    }).id();

    // === SPAWN PLAYERS AND PADDLES ===
    // Player 1 (Left side)
    let paddle1 = commands.spawn(
        PaddleBundle::new(
            &mut meshes,
            &mut materials,
            Vec3::new(-600.0, 0.0, 0.0),
            PADDLE_SIZE
        )
    ).id();

    commands.spawn(PlayerBundle::new(
        Player {
            id: 1,
            team: team1,
            name: "Player 1".into(),
        },
        paddle1
    ));

    // Player 2 (Right side)
    let paddle2 = commands.spawn(
        PaddleBundle::new(
            &mut meshes,
            &mut materials,
            Vec3::new(600.0, 0.0, 0.0),
            PADDLE_SIZE
        )
    ).id();

    commands.spawn(PlayerBundle::new(
        Player {
            id: 2,
            team: team2,
            name: "Player 2".into(),
        },
        paddle2
    ));

    // === SPAWN BALL ===
    commands.spawn(BallBundle::new(
        &mut meshes,
        &mut materials,
        Vec3::ZERO,
        Vec2::new(-300.0, 300.0)
    ));

    // === SPAWN AREA BOUNDARIES (Walls) ===
    let wall_thickness = 20.0;
    let half_width = FIXED_DIMENSIONS.x / 2.0;
    let half_height = FIXED_DIMENSIONS.y / 2.0;

    // Top Wall
    commands.spawn((
        Wall { side: Side::Top },
        Mesh2d(meshes.add(Rectangle::new(FIXED_DIMENSIONS.x, wall_thickness))),
        MeshMaterial2d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
        Transform::from_xyz(0.0, half_height + wall_thickness / 2.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(FIXED_DIMENSIONS.x, wall_thickness),
    ));

    // Bottom Wall
    commands.spawn((
        Wall { side: Side::Bottom },
        Mesh2d(meshes.add(Rectangle::new(FIXED_DIMENSIONS.x, wall_thickness))),
        MeshMaterial2d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
        Transform::from_xyz(0.0, -half_height - wall_thickness / 2.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(FIXED_DIMENSIONS.x, wall_thickness),
    ));

    // === SPAWN GOALS ===
    let goal_height = 300.0;

    // Left Goal (Team 1 defends, Team 2 scores here)
    commands.spawn((
        Goal { team_id: 1 },
        Sensor,
        Collider::rectangle(20.0, goal_height),
        Transform::from_xyz(-half_width, 0.0, 0.0),
        CollisionLayers::default(),
    ));

    // Right Goal (Team 2 defends, Team 1 scores here)
    commands.spawn((
        Goal { team_id: 0 },
        Sensor,
        Collider::rectangle(20.0, goal_height),
        Transform::from_xyz(half_width, 0.0, 0.0),
        CollisionLayers::default(),
    ));
}