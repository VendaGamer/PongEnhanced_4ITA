use bevy::prelude::{Entity, EntityEvent};

#[derive(EntityEvent)]
pub struct ButtonPressed(pub Entity);

#[derive(EntityEvent)]
pub struct SelectorValueChanged(pub Entity);