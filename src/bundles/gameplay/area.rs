use crate::bundles::paddle::PaddleBundle;
use crate::bundles::wall::WallBundle;
use crate::bundles::{BallBundle, GoalBundle};
use crate::components::area::Area;
use crate::components::game_modes::{FlashyLight, PaddleTilt};
use crate::models::game::gameplay::GameMode;
use crate::resources::GameModeConfig;
use crate::systems::handle_scoring;
use crate::utils::{BALL_RADIUS, FIXED_DIMENSIONS, PADDLE_SIZE};
use bevy::prelude::*;
use bevy_light_2d::prelude::PointLight2d;

#[derive(Bundle)]
pub struct AreaBundle {
    pub area: Area,
    pub transform: Transform,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl AreaBundle {
    pub fn spawn(
        config: &GameModeConfig,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) {
        commands.spawn(BallBundle::new(
            meshes,
            materials,
            Vec3::ZERO,
            Vec2::new(-300.0, 300.0),
            BALL_RADIUS
        )).observe(handle_scoring);

        match config.game_mode {
            GameMode::Classic =>{

            },
            GameMode::Modern => {

            },
            GameMode::Blackout => {
                
                commands.spawn((
                    Transform::from_translation(Vec3::ZERO),
                    FlashyLight,
                    PointLight2d{
                        color: Color::srgb(1.0, 1.0, 1.0),
                        radius: 500.0,
                        intensity: 2.0,
                        ..default()
                    }
                ));
            },
            GameMode::Twisted => {

            },
            GameMode::UpsideDown => {

            }
        }


        let teams = config.area_shape.get_teams();

        for team in teams {
            let goal = commands.spawn(GoalBundle::new(team)).id();
            let positions = team.get_positions();

            for i in 0..team.players.len() {
                let mut paddle =
                commands.spawn(PaddleBundle::new(meshes, materials, positions[i], PADDLE_SIZE, goal, team.players[i]));

                if matches!(config.game_mode, GameMode::Twisted){
                    paddle.insert(PaddleTilt {tilt: 0.0});
                }
            }
            
            team.area_side.spawn_score_text(commands);
        }

        let walls = config.area_shape.get_wall_sides();
        for side in walls {
            commands.spawn(WallBundle::new(*side));
        }

        commands.spawn(AreaBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            area: Area,
            mesh: Mesh2d(meshes.add(Rectangle::new(FIXED_DIMENSIONS.x, FIXED_DIMENSIONS.y))),
            material: MeshMaterial2d(materials.add(Color::srgb(0.05, 0.05, 0.05))),
        });
    }
}