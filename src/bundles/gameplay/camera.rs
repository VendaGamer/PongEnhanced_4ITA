use crate::utils::FIXED_DIMENSIONS;
use bevy::camera;
use bevy::color::Color;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct CameraBundle {
    pub camera2d: Camera2d,
    pub camera: Camera,
    pub projection: Projection,
    pub ui_anti_alias: UiAntiAlias,
    pub msaa: Msaa,
}

impl Default for CameraBundle {
    fn default() -> Self {
        Self {
            camera2d: Camera2d,
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            projection: OrthographicProjection {
                scaling_mode: camera::ScalingMode::Fixed {
                    width: FIXED_DIMENSIONS.x,
                    height: FIXED_DIMENSIONS.y,
                },
                ..OrthographicProjection::default_2d()
            }.into(),
            ui_anti_alias: UiAntiAlias::Off,
            msaa: Msaa::Off,
        }
    }
}