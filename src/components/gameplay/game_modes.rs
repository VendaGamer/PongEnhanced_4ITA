use crate::bundles::Component;

#[derive(Component)]
pub struct PlayerHealth{
    player_health: i32
}

pub const PUSH_COOLDOWN: f32 = 5.0;

#[derive(Component)]
pub struct PaddlePush {
    pub current_cooldown: f32,
}