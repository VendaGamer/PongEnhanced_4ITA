use avian2d::prelude::Collider;
use bevy::prelude::*;
use crate::utils::*;

#[derive(Component, Clone, Copy)]
pub struct Area{
    pub shape: AreaShape
}

#[derive(Component)]
pub struct DivisionLine;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum AreaShape {
    TwoSide,
    Triangular,
    Cuboid,
}

#[derive(Clone, Copy)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}


impl Side {
    pub fn get_transform(side: Self) -> Transform {


        match side {
            Side::Left => Transform {
                translation: Vec3::new(-HALF_WIDTH - HALF_WALL_THICKNESS, 0.0, 0.0),
                scale: Vec3::new(WALL_THICKNESS, FIXED_DIMENSIONS.y, 1.0),
                ..default()
            },
            Side::Right => Transform {
                translation: Vec3::new(HALF_WIDTH + HALF_WALL_THICKNESS, 0.0, 0.0),
                scale: Vec3::new(WALL_THICKNESS, FIXED_DIMENSIONS.y, 1.0),
                ..default()
            },
            Side::Top => Transform {
                translation: Vec3::new(0.0, HALF_HEIGHT + HALF_WALL_THICKNESS, 0.0),
                scale: Vec3::new(FIXED_DIMENSIONS.x, WALL_THICKNESS, 1.0),
                ..default()
            },
            Side::Bottom => Transform {
                translation: Vec3::new(0.0, -HALF_HEIGHT - HALF_WALL_THICKNESS / 2.0, 0.0),
                scale: Vec3::new(FIXED_DIMENSIONS.x, WALL_THICKNESS, 1.0),
                ..default()
            },
        }
    }

    pub fn get_collider(side: Self) -> Collider {
        match side {
            Side::Left | Side::Right => {
                Collider::rectangle(WALL_THICKNESS, FIXED_DIMENSIONS.y)
            }
            Side::Top | Side::Bottom => {
                Collider::rectangle(FIXED_DIMENSIONS.x, WALL_THICKNESS)
            }
        }
    }
}

#[derive(Component)]
pub struct Goal {
    pub team: Entity,
    pub side: Side
}

#[derive(Component)]
pub struct Wall {
    pub side: Side,
}