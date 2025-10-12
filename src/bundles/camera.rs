use bevy::color::Color;
use bevy::prelude::*;
use bevy::render::camera;

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
                scaling_mode: camera::ScalingMode::Fixed {
                    width: 1280.0,
                    height: 720.0,
                },
                ..OrthographicProjection::default_2d()
            }),
        }
    }
}