use bevy::color::Color;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::utils::FIXED_DIMENSIONS;

#[derive(Bundle)]
pub struct CameraBundle {
    pub camera2d: Camera2d,
    pub camera: Camera,
    pub projection: Projection,
}

impl Default for CameraBundle {
    fn default() -> Self {
        Self {
            camera2d: Camera2d,
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            projection: Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::Fixed {
                    width: FIXED_DIMENSIONS.x,
                    height: FIXED_DIMENSIONS.y,
                },
                ..OrthographicProjection::default_2d()
            }),
        }
    }
}