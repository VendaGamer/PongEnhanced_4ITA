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

pub const PUSH_COOLDOWN: f32 = 7.0;
pub const SPEEDUP_COOLDOWN: f32 = 7.0;