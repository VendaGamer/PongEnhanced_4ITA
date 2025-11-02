use bevy::prelude::Component;
use crate::components::area_shape::AreaShape;
use crate::components::team::Team;

#[derive(Component)]
pub struct Area{
    pub shape: AreaShape
}