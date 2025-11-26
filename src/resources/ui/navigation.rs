use bevy::prelude::Resource;

#[derive(Resource)]
pub struct UISelection {
    pub row: i32,
    pub column: i32,
}

impl Default for UISelection{
    fn default() -> Self {
        Self{row: 0, column: 0}
    }
}