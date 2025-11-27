use bevy::prelude::Component;
use crate::bundles::{App, Entity, Vec2};

#[derive(Component)]
pub struct Ball{
    pub initial_velocity: Vec2
}

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

#[derive(Component, Copy, Clone)]
pub struct Paddle{
    pub player: Entity
}

#[derive(Component)]
pub struct Team {
    pub name: String,
    pub current_score: u32
}