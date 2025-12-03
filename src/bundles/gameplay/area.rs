use bevy::prelude::*;
use crate::components::area::Area;
use crate::components::*;
use crate::bundles::GoalBundle;
use crate::bundles::wall::WallBundle;
use crate::utils::screen::TRANSFORM_ZERO;

#[derive(Bundle)]
pub struct AreaBundle {
    pub area: Area,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl AreaBundle {
    pub fn spawn(
        area_shape: AreaShape,
        commands: &mut Commands,
    ){

        match area_shape {
            AreaShape::TwoSide(teams) => {
                commands.spawn(GoalBundle::new(teams[0], Side::Left));
                commands.spawn(GoalBundle::new(teams[1], Side::Right));
                commands.spawn(WallBundle::new(Side::Bottom));
                commands.spawn(WallBundle::new(Side::Top));
            }
            AreaShape::Triangular(teams) => {
                commands.spawn(GoalBundle::new(teams[0], Side::Left));
                commands.spawn(GoalBundle::new(teams[1], Side::Right));
                commands.spawn(GoalBundle::new(teams[2], Side::Bottom));
                commands.spawn(WallBundle::new(Side::Top));
            }
            AreaShape::Cuboid(teams) => {
                commands.spawn(GoalBundle::new(teams[0], Side::Left));
                commands.spawn(GoalBundle::new(teams[1], Side::Right));
                commands.spawn(GoalBundle::new(teams[2], Side::Bottom));
                commands.spawn(GoalBundle::new(teams[3], Side::Top));
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