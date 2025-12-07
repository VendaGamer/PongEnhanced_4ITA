use crate::bundles::paddle::PaddleBundle;
use crate::bundles::wall::WallBundle;
use crate::bundles::GoalBundle;
use crate::components::area::Area;
use crate::models::game::area::{AreaShape, AreaSide};
use crate::utils::screen::TRANSFORM_ZERO;
use bevy::prelude::*;
use crate::bundles::player::PlayerBundle;
use crate::utils::PADDLE_SIZE;

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
        let teams = area_shape.get_teams();

        for team in teams {
            let goal = commands.spawn(GoalBundle::new(team)).id();
            let positions = team.get_positions();

            for position in positions {
                commands.spawn(PaddleBundle::new(meshes, materials, position, PADDLE_SIZE, goal));
            }

        }

        commands.spawn(AreaBundle {
            transform: TRANSFORM_ZERO,
            area: Area,
            global_transform: GlobalTransform::from(TRANSFORM_ZERO)
        });
    }
}