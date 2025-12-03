use avian2d::prelude::Collider;
use bevy::prelude::*;
use crate::components::Paddle;
use crate::models::game::area::{AreaShape, AreaSide};
use crate::utils::*;

#[derive(Component)]
pub struct Area {
    pub shape: AreaShape
}

#[derive(Component)]
pub struct DivisionLine;

#[derive(Component)]
pub struct Goal {
    pub side: AreaSide,
    pub paddles: Vec<Entity>,
}

#[derive(Component)]
pub struct Wall {
    pub side: AreaSide,
}