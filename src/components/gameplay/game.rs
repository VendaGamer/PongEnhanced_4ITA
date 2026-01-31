use crate::bundles::{App, Entity, Vec2};
use crate::models::game::area::LocalPlayerID;
use bevy::prelude::Component;
use crate::models::game::gameplay::PlayerId;

#[derive(Component)]
pub struct Ball {
    pub initial_velocity: Vec2
}

pub trait GameModeRules: Send + Sync {
    fn ball_speed(&self) -> f32;
    fn gravity_scale(&self) -> f32;
    fn paddle_speed_multiplier(&self) -> f32;
    fn apply_special_mechanics(&self, app: &mut App);
}

#[derive(Component, Copy, Clone)]
pub struct Paddle {
    pub goal: Entity,
    pub id: PlayerId,
}