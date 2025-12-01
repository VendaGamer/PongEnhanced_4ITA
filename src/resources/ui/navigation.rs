use bevy::prelude::Resource;

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