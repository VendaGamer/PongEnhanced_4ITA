use bevy::math::Quat;
use crate::bundles::Component;

#[derive(Component)]
pub struct PlayerHealth {
    player_health: i32
}

#[derive(Component)]
pub struct PaddlePush {
    pub current_cooldown: f32,
}

#[derive(Component)]
pub struct PaddleSpeedup {
    pub current_cooldown: f32,
}

#[derive(Component)]
pub struct FlashyLight;

#[derive(Component)]
pub struct PaddleTilt {
    pub tilt: f32,
}

pub const PUSH_COOLDOWN: f32 = 7.0;
pub const SPEEDUP_COOLDOWN: f32 = 7.0;

pub const MAX_ABS_TILT: f32 = 15.0;