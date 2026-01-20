use crate::bundles::paddle::PaddleBundle;
use crate::bundles::wall::WallBundle;
use crate::bundles::{BallBundle, GoalBundle};
use crate::components::area::Area;
use crate::models::game::area::AreaShape;
use crate::utils::screen::TRANSFORM_ZERO;
use crate::utils::{BALL_RADIUS, PADDLE_SIZE};
use bevy::prelude::*;
use crate::systems::handle_scoring;

#[derive(Bundle)]
pub struct AreaBundle {
    pub area: Area,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl AreaBundle {
    pub fn spawn(
        area_shape: &AreaShape,
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
        
        let teams = area_shape.get_teams();

        for team in teams {
            let goal = commands.spawn(GoalBundle::new(team)).id();
            let positions = team.get_positions();

            for i in 0..team.players.len() {
                commands.spawn(PaddleBundle::new(meshes, materials, positions[i],
                                                 PADDLE_SIZE, goal, team.players[i]));
            }
            
            team.area_side.spawn_score_text(commands);
        }

        let walls = area_shape.get_wall_sides();
        for side in walls {
            commands.spawn(WallBundle::new(*side));
        }
        
        commands.spawn(AreaBundle {
            transform: TRANSFORM_ZERO,
            area: Area,
            global_transform: GlobalTransform::from(TRANSFORM_ZERO)
        });
    }
}