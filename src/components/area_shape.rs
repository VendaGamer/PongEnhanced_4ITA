use bevy::prelude::Component;

#[derive(Component)]
pub enum AreaShape {
    TwoSide,
    Triangular,
    Cuboid,
}