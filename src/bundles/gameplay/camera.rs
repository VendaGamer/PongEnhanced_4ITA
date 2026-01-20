use crate::utils::FIXED_DIMENSIONS;
use bevy::camera;
use bevy::color::Color;
use bevy::prelude::*;
use bevy_light_2d::light::AmbientLight2d;
use bevy_light_2d::prelude::Light2d;

#[derive(Bundle)]
pub struct CameraBundle {
    pub camera2d: Camera2d,
    pub light: Light2d,
    pub camera: Camera,
    pub projection: Projection,
    pub ui_anti_alias: UiAntiAlias,
    pub msaa: Msaa,
}

impl Default for CameraBundle {
    fn default() -> Self {
        Self {
            camera2d: Camera2d,
            light: Light2d {
                ambient_light: AmbientLight2d {
                    brightness: 1.0,
                    color: Color::WHITE,
                }
            },
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