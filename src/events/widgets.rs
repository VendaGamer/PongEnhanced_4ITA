use bevy::prelude::{Entity, EntityEvent};

#[derive(EntityEvent)]
pub struct ButtonPressed(pub Entity);

#[derive(EntityEvent)]
pub struct SelectorValueChanged(pub Entity);

#[derive(EntityEvent)]
pub struct OptionChanged(pub Entity);

#[derive(EntityEvent)]
pub struct SliderValueChanged{
    pub entity: Entity,
    pub value: f32,
}