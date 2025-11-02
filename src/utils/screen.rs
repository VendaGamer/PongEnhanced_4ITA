use bevy::prelude::*;
use avian2d::prelude::LinearDamping;

pub const FIXED_DIMENSIONS: Vec2 = Vec2::new(1280.0, 720.0);
pub const ZERO_DAMPING: LinearDamping = LinearDamping(0.0);
pub const PADDLE_SIZE: Vec2 = Vec2::new(25.0 / 2.0, 200.0 / 2.0);
pub const BALL_RADIUS: f32 = 7.0;