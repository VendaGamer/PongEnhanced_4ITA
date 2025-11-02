use bevy::prelude::*;
use avian2d::prelude::LinearDamping;

pub const FIXED_DIMENSIONS: Vec2 = Vec2::new(1280.0, 720.0);
pub const ZERO_DAMPING: LinearDamping = LinearDamping(0.0);