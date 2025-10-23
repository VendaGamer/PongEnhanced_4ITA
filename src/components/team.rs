use bevy::prelude::Component;

#[derive(Component)]
pub struct Team {
    pub id: u8,
    pub name: String,
    pub current_score: u32
}