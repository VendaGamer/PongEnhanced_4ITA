use bevy::prelude::Resource;
use crate::components::area_shape::AreaShape;

#[derive(Resource)]
pub struct GameConfig {
    pub area_shape: AreaShape,
    pub player_count: u8,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            area_shape: AreaShape::TwoSide,
            player_count: 2,
        }
    }
}