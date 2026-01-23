use bevy::prelude::{Entity, EntityEvent};

#[derive(EntityEvent)]
pub struct ButtonPressed(pub Entity);

#[derive(EntityEvent)]
pub struct OptionChanged {
    pub entity: Entity,
    pub selected_index: usize,
}

#[derive(EntityEvent)]
pub struct SliderValueChanged{
    pub entity: Entity,
    pub value: f32,
}

#[derive(EntityEvent)]
pub struct CheckboxChanged {
    pub entity: Entity,
    pub state: bool,
}