use bevy::prelude::*;
use crate::components::area::Area;
use crate::bundles::GoalBundle;
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
    ){
        match area_shape {
            AreaShape::TwoSide(Some(teams)) => {

                commands.spawn_batch([
                    GoalBundle::new(teams[0]., AreaSide::Left),
                    GoalBundle::new(teams[1], AreaSide::Right),
                ]);
                commands.spawn([
                    WallBundle::new(AreaSide::Bottom),
                    WallBundle::new(AreaSide::Top)
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