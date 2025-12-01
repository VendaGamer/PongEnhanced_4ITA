use bevy::prelude::Component;
use crate::models::ui::option::UIOption;

#[derive(Component)]
pub struct Dropdown
{
    pub options: Vec<UIOption>,
    pub selected: usize,
}