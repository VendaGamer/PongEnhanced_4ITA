use bevy::prelude::Component;

#[derive(Component)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}