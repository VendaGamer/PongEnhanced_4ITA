use bevy::app::App;

#[derive(Clone, Copy)]
pub enum GameMode {
    Classic,
    UpsideDown,
    Modern,
    BlackOut,
    Twisted,
}

pub trait GameModeRules: Send + Sync {
    fn ball_speed(&self) -> f32;
    fn gravity_scale(&self) -> f32;
    fn paddle_speed_multiplier(&self) -> f32;
    fn apply_special_mechanics(&self, app: &mut App);
}