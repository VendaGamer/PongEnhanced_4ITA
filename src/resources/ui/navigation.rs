use bevy::prelude::Resource;
use crate::bundles::Vec2;

#[derive(Resource)]
pub struct UISelection {
    pub row: u32,
    pub column: u32,
}

impl Default for UISelection{
    fn default() -> Self {
        Self{row: 0, column: 0}
    }
}

#[derive(Resource, Default)]
pub struct NavigationState {
    pub(crate) last_axis: Vec2,
    pub(crate) cooldown: f32,
}