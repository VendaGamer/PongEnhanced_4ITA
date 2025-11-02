use bevy::prelude::Component;
use crate::bundles::Vec2;

#[derive(Component)]
pub struct Ball{
    pub initial_velocity: Vec2
}