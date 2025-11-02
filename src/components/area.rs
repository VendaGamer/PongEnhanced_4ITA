use crate::components::area_shape::AreaShape;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Area{
    pub shape: AreaShape
}