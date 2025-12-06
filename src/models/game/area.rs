use crate::bundles::{default, Entity, Transform, Vec3};
use crate::utils::{FIXED_DIMENSIONS, HALF_HEIGHT, HALF_WALL_THICKNESS, HALF_WIDTH, WALL_THICKNESS};
use avian2d::prelude::Collider;
use bevy::prelude::Resource;
use AreaShape::{Cuboid, Triangular, TwoSide};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum AreaSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Team {
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
    TwoSide(Option<[Team; 2]>),
    Triangular(Option<[Team; 3]>),
    Cuboid(Option<[Team; 4]>),
}

impl AreaShape {

    pub fn get_team(&self, goal: Entity) -> Option<&Team> {

        let teams = self.get_teams();

        for team in teams.iter(){
            if let Some(team_goal) = team.goal{
                if team_goal == goal{
                    return Some(&team);
                }
            }
        }
        None

    }

    pub fn get_team_mut(&mut self, goal: Entity) -> Option<&mut Team> {

        let teams = self.get_teams_mut();

        for team in teams.iter_mut(){
            if let Some(team_goal) = team.goal{
                if team_goal == goal{
                    return Some(team);
                }
            }
        }
        None

    }

    pub fn get_teams(&self) -> &[Team] {
        match self {
            TwoSide(opt)     => opt.as_ref().unwrap(),
            Triangular(opt)  => opt.as_ref().unwrap(),
            Cuboid(opt)      => opt.as_ref().unwrap(),
        }
    }

    pub fn get_teams_mut(&mut self) -> &mut [Team] {
        match self {
            TwoSide(opt)     => opt.as_mut().unwrap(),
            Triangular(opt)  => opt.as_mut().unwrap(),
            Cuboid(opt)      => opt.as_mut().unwrap(),
        }
    }
}

impl AreaSide {

    pub fn get_transform(self) -> Transform {


        match self {
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

    pub fn get_paddle_pos(self, player_count: u32) -> Vec<Vec3> {
        if player_count == 0 {
            return Vec::new();
        }

        let mut positions = Vec::with_capacity(player_count as usize);

        match self {
            AreaSide::Left | AreaSide::Right => {
                let x = if self == AreaSide::Left {
                    -HALF_WIDTH + 50.0
                } else {
                    HALF_WIDTH - 50.0
                };

                // Evenly distribute along the height
                let spacing = FIXED_DIMENSIONS.y / (player_count + 1) as f32;

                for i in 0..player_count {
                    let y = -HALF_HEIGHT + spacing * (i + 1) as f32;
                    positions.push(Vec3::new(x, y, 0.0));
                }
            },
            AreaSide::Top | AreaSide::Bottom => {
                let y = if self == AreaSide::Top {
                    HALF_HEIGHT - 50.0
                } else {
                    -HALF_HEIGHT + 50.0
                };

                // Evenly distribute along the width
                let spacing = FIXED_DIMENSIONS.x / (player_count + 1) as f32;

                for i in 0..player_count {
                    let x = -HALF_WIDTH + spacing * (i + 1) as f32;
                    positions.push(Vec3::new(x, y, 0.0));
                }
            }
        }

        positions
    }

    pub fn get_collider(self) -> Collider {
        match self {
            AreaSide::Left | AreaSide::Right => {
                Collider::rectangle(WALL_THICKNESS, FIXED_DIMENSIONS.y)
            }
            AreaSide::Top | AreaSide::Bottom => {
                Collider::rectangle(FIXED_DIMENSIONS.x, WALL_THICKNESS)
            }
        }
    }
}