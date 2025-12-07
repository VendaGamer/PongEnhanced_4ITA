use crate::models::game::area::{AreaShape, AreaSide};
use bevy::prelude::*;

#[derive(Component)]
pub struct Area;

#[derive(Component)]
pub struct DivisionLine;

#[derive(Component)]
pub struct Goal {
    pub side: AreaSide,
}

#[derive(Component)]
pub struct Wall {
    pub side: AreaSide,
}