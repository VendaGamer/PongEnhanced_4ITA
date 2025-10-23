use bevy::prelude::*;
use crate::components::area_shape::AreaShape;

#[derive(Bundle)]
pub struct AreaBundle {
    pub shape: AreaShape,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl AreaBundle{
    pub fn new(shape: AreaShape) -> Self {
        AreaBundle {
            shape,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            global_transform: GlobalTransform::default(),
        }
    }
}

impl Default for AreaBundle {
    fn default() -> Self {
        Self {
            shape: AreaShape::TwoSide,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}