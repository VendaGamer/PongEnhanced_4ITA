use crate::bundles::widgets::LabelBundle;
use crate::bundles::{default, Entity, Transform, Vec3};
use crate::components::ui::{ScoreText, UIOptionString};
use crate::models::game::gameplay::PlayerId;
use crate::utils::{FIXED_DIMENSIONS, HALF_HEIGHT, HALF_WALL_THICKNESS, HALF_WIDTH, WALL_THICKNESS};
use avian2d::prelude::Collider;
use bevy::prelude::{Color, Commands, Node, PositionType, Reflect, Vec2};
use bevy::ui::Val;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use AreaShape::{Cuboid, Triangular, TwoSide};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub enum AreaSide {
    Left,
    Right,
    Top,
    Bottom,
}


impl AreaSide{
    pub fn opposite(self) -> AreaSide {
        match self {
            AreaSide::Left => AreaSide::Right,
            AreaSide::Right => AreaSide::Left,
            AreaSide::Top => AreaSide::Bottom,
            AreaSide::Bottom => AreaSide::Top,
        }
    }

    pub fn spawn_score_text(self, commands: &mut Commands){

        let position = match self {

            AreaSide::Left => Vec2::new(FIXED_DIMENSIONS.x - (FIXED_DIMENSIONS.x / 3.0), 0.0),
            AreaSide::Right => Vec2::new(FIXED_DIMENSIONS.x / 3.0, 0.0),
            AreaSide::Top => Vec2::new(0.0, HALF_HEIGHT),
            AreaSide::Bottom => Vec2::new(0.0, -HALF_HEIGHT),
        };

        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(position.x),
                top: Val::Px(position.y),
                ..default()
            },
            LabelBundle::custom("0", Color::WHITE.into(), 80.0),
            ScoreText {
                area_side: self,
            }
        ));
    }

    pub fn is_vertical(self) -> bool {
        match self {
            AreaSide::Left | AreaSide::Right => true,
            AreaSide::Top | AreaSide::Bottom => false,
        }
    }
}


#[derive(Clone, Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub struct TeamInfo {
    pub current_score: u32,
    pub area_side: AreaSide,
    pub players: Vec<PlayerId>,
}

impl TeamInfo {
    pub fn get_positions(&self) -> Vec<Vec3> {
        self.area_side.get_paddle_pos(self.players.len())
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect, Eq, Hash)]
pub enum LocalPlayerID {
    Keyboard(u8),
    Gamepad(Entity)
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
pub enum AreaShape {
    TwoSide([TeamInfo; 2]),
    Triangular([TeamInfo; 3]),
    Cuboid([TeamInfo; 4]),
}

impl AreaShape {

    pub fn default() -> AreaShape {

        TwoSide([
            TeamInfo {
                current_score: 0,
                area_side: AreaSide::Left,
                players: Vec::new(),
            },
            TeamInfo {
                current_score: 0,
                area_side: AreaSide::Right,
                players: Vec::new(),
            },
        ])

    }
}

impl UIOptionString for AreaShape {
    fn push_ui_option_string(&self, string: &mut String) {
        let s = match self {
            TwoSide(_) => "Two Side",
            Triangular(_) => "Triangular",
            Cuboid(_) => "Cuboid",
        };
        string.push_str(s);
    }
}


impl AreaShape {


    pub fn get_wall_sides(&self) -> &[AreaSide] {
        match self {
            TwoSide(_) => &[AreaSide::Top, AreaSide::Bottom],
            Triangular(_) => &[],
            Cuboid(_) => &[],
        }
    }
    pub fn get_team(&mut self, side: AreaSide) -> Option<&TeamInfo> {

        let teams = self.get_teams();

        for team in teams.iter(){
            if side == team.area_side {
                return Some(team);
            }
        }
        None

    }

    pub fn get_team_mut(&mut self, side: AreaSide) -> Option<&mut TeamInfo> {

        let teams = self.get_teams_mut();

        for team in teams.iter_mut(){
            if side == team.area_side {
                return Some(team);
            }
        }
        
        None
    }
    
    pub fn contains_player(&self, player_id: PlayerId) -> bool {
        for team in self.get_teams().iter(){
            for player in team.players.iter(){
                if *player == player_id {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_teams(&self) -> &[TeamInfo] {
        match self {
            TwoSide(opt)     => opt.as_ref(),
            Triangular(opt)  => opt.as_ref(),
            Cuboid(opt)      => opt.as_ref(),
        }
    }

    pub fn get_teams_mut(&mut self) -> &mut [TeamInfo] {
        match self {
            TwoSide(opt)     => opt.as_mut(),
            Triangular(opt)  => opt.as_mut(),
            Cuboid(opt)      => opt.as_mut(),
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

    pub fn get_paddle_pos(self, player_count: usize) -> Vec<Vec3> {
        if player_count == 0 {
            return Vec::new();
        }

        let mut positions = Vec::with_capacity(player_count);

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