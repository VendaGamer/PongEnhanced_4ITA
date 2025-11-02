use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct ControlledPaddle{
    pub paddle: Entity,
}

impl ControlledPaddle {
    pub fn new(paddle: Entity) -> Self{
        Self{
            paddle
        }
    }
}