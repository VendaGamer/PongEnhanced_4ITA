use bevy::prelude::*;
use crate::components::area::Area;
use crate::bundles::GoalBundle;
use crate::bundles::paddle::PaddleBundle;
use crate::bundles::wall::WallBundle;
use crate::components::Player;
use crate::models::game::area::{AreaShape, AreaSide};
use crate::resources::GameConfig;
use crate::utils::screen::TRANSFORM_ZERO;

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
    ){
        match area_shape {
            AreaShape::TwoSide(Some(teams)) => {
                let team1 = &teams[0];
                let team2 = &teams[1];
                let goal1 = commands.spawn(GoalBundle::new(team1)).id();
                let goal2 = commands.spawn(GoalBundle::new(team2)).id();

                commands.spawn_batch([
                    PaddleBundle::new(meshes, materials, team1.area_side, goal1),
                    PaddleBundle::new(meshes, materials, team2.area_side, goal2),
                ]);


                commands.spawn_batch([
                    WallBundle::new(AreaSide::Bottom),
                    WallBundle::new(AreaSide::Top),
                ]);
            }
            AreaShape::Triangular(Some(teams)) => {
                commands.spawn(GoalBundle::new(teams[0], AreaSide::Left));
                commands.spawn(GoalBundle::new(teams[1], AreaSide::Right));
                commands.spawn(GoalBundle::new(teams[2], AreaSide::Bottom));
                commands.spawn(WallBundle::new(AreaSide::Top));
            }
            AreaShape::Cuboid(Some(teams)) => {
                commands.spawn(GoalBundle::new(teams[0], AreaSide::Left));
                commands.spawn(GoalBundle::new(teams[1], AreaSide::Right));
                commands.spawn(GoalBundle::new(teams[2], AreaSide::Bottom));
                commands.spawn(GoalBundle::new(teams[3], AreaSide::Top));
            }
        }

        commands.spawn(AreaBundle{
            transform: TRANSFORM_ZERO,
            area: Area{
                shape: area_shape
            },
            global_transform: GlobalTransform::from(TRANSFORM_ZERO)
        });
    }
}