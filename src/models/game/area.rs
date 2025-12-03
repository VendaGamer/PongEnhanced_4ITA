use std::ops::Deref;
use crate::bundles::{default, Entity, Transform, Vec3};
use crate::utils::{FIXED_DIMENSIONS, HALF_HEIGHT, HALF_WALL_THICKNESS, HALF_WIDTH, WALL_THICKNESS};
use avian2d::prelude::Collider;
use bevy::prelude::{Deref, Resource};
use AreaShape::{Cuboid, Triangular, TwoSide};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum AreaSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct TeamConfig {
    pub name: String,
    pub current_score: u32,
    pub area_side: AreaSide,
    pub goal: Option<Entity>,
    pub players: Vec<Player>,
}

#[derive(Resource)]
pub struct Players {
    pub players: Vec<Player>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Player {
    pub name: String,
    pub entity: Option<Entity>,
}

pub enum ControlType{
    Keyboard,
    Gamepad(Entity),
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum AreaShape {
    TwoSide(Option<[TeamConfig; 2]>),
    Triangular(Option<[TeamConfig; 3]>),
    Cuboid(Option<[TeamConfig; 4]>),
}

impl AreaShape {
    pub fn get_teams(&self) -> &[TeamConfig] {
        match self {
            TwoSide(opt)     => opt.as_ref().unwrap(),
            Triangular(opt)  => opt.as_ref().unwrap(),
            Cuboid(opt)      => opt.as_ref().unwrap(),
        }
    }
}

impl AreaSide {

    pub fn get_transform(side: Self) -> Transform {


        match side {
            AreaSide::Left => Transform {
                translation: Vec3::new(-HALF_WIDTH - HALF_WALL_THICKNESS, 0.0, 0.0),
                scale: Vec3::new(WALL_THICKNESS, FIXED_DIMENSIONS.y, 1.0),
                ..default()
            },
            AreaSide::Right => Transform {
                translation: Vec3::new(HALF_WIDTH + HALF_WALL_THICKNESS, 0.0, 0.0),
                scale: Vec3::new(WALL_THICKNESS, FIXED_DIMENSIONS.y, 1.0),
                ..default()
            },
            AreaSide::Top => Transform {
                translation: Vec3::new(0.0, HALF_HEIGHT + HALF_WALL_THICKNESS, 0.0),
                scale: Vec3::new(FIXED_DIMENSIONS.x, WALL_THICKNESS, 1.0),
                ..default()
            },
            AreaSide::Bottom => Transform {
                translation: Vec3::new(0.0, -HALF_HEIGHT - HALF_WALL_THICKNESS / 2.0, 0.0),
                scale: Vec3::new(FIXED_DIMENSIONS.x, WALL_THICKNESS, 1.0),
                ..default()
            },
        }
    }

    pub fn get_collider(side: Self) -> Collider {
        match side {
            AreaSide::Left | AreaSide::Right => {
                Collider::rectangle(WALL_THICKNESS, FIXED_DIMENSIONS.y)
            }
            AreaSide::Top | AreaSide::Bottom => {
                Collider::rectangle(FIXED_DIMENSIONS.x, WALL_THICKNESS)
            }
        }
    }
}