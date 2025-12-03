use std::hash::Hash;
use crate::models::ui::option::UIOption;
use bevy::prelude::*;

#[derive(Component)]
pub struct Dropdown
{
    pub options: Vec<UIOption>,
    pub selected: usize,
}

#[derive(Component)]
#[require(Button)]
pub struct SelectorButton {
    pub selector: Entity
}