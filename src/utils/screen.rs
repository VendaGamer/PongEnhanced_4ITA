use bevy::prelude::*;
use avian2d::prelude::LinearDamping;

pub const FIXED_DIMENSIONS: Vec2 = Vec2::new(1280.0, 720.0);
pub const ZERO_DAMPING: LinearDamping = LinearDamping(0.0);
pub const PADDLE_SIZE: Vec2 = Vec2::new(12.5, 100.0);
pub const BALL_RADIUS: f32 = 7.0;
pub const WALL_THICKNESS: f32 = 0.0;
pub const HALF_WIDTH: f32 = FIXED_DIMENSIONS.x / 2.0;
pub const HALF_HEIGHT: f32 = FIXED_DIMENSIONS.y / 2.0;
pub const HALF_WALL_THICKNESS: f32 = WALL_THICKNESS / 2.0;

pub const PADDLE_WALL_PADDING: f32 = 25.0;
pub const TRANSFORM_ZERO: Transform = Transform::from_xyz(0.0,0.0,0.0);