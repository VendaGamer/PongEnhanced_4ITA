use bevy::prelude::*;
use crate::components::area::Area;
use crate::components::AreaShape;
use crate::bundles::GoalBundle;
use crate::bundles::wall::WallBundle;
use crate::components::side::Side;
use crate::utils::screen::TRANSFORM_ZERO;

#[derive(Bundle, Clone, Copy)]
pub struct AreaBundle {
    pub area: Area,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl AreaBundle {
    pub fn spawn(
        area_shape: AreaShape,
        commands: &mut Commands,
        teams: &[Entity],
    ) -> Entity {


        match area_shape {
            AreaShape::TwoSide => {
                if teams.len() >= 2 {
                    commands.spawn(GoalBundle::new(teams[0], Side::Left));
                    commands.spawn(GoalBundle::new(teams[1], Side::Right));
                    commands.spawn(WallBundle::new(Side::Bottom));
                    commands.spawn(WallBundle::new(Side::Top));

                }
            }
            AreaShape::Triangular => {
                if teams.len() >= 3 {
                    commands.spawn(GoalBundle::new(teams[0], Side::Left));
                    commands.spawn(GoalBundle::new(teams[1], Side::Right));
                    commands.spawn(GoalBundle::new(teams[2], Side::Bottom));
                    commands.spawn(WallBundle::new(Side::Top));
                }
            }
            AreaShape::Cuboid => {
                if teams.len() >= 4 {
                    commands.spawn(GoalBundle::new(teams[0], Side::Left));
                    commands.spawn(GoalBundle::new(teams[1], Side::Right));
                    commands.spawn(GoalBundle::new(teams[2], Side::Bottom));
                    commands.spawn(GoalBundle::new(teams[3], Side::Top));
                }
            }
        }

        commands.spawn(AreaBundle{
            transform: TRANSFORM_ZERO,
            area: Area{
                shape: area_shape
            },
            global_transform: GlobalTransform::from(TRANSFORM_ZERO)
        }).id()
    }
}