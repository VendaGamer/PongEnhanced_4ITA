use crate::components::area_shape::AreaShape;
use bevy::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct Area{
    pub shape: AreaShape
}