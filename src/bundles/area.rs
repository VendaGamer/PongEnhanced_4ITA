use bevy::prelude::*;
use crate::components::area::Area;
use crate::components::area_shape::AreaShape;

#[derive(Bundle)]
pub struct AreaBundle {
    pub area: Area,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl AreaBundle{
    pub fn new(area_shape: AreaShape) -> Self {
        AreaBundle {
            area: Area{
                shape: area_shape
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl Default for AreaBundle {
    fn default() -> Self {
        Self {
            area: Area{
                shape: AreaShape::Cuboid
            },
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}