use bevy::prelude::Component;

#[derive(Component)]
pub struct Dropdown<T>
    where T: Copy + 'static + Send + Sync,
{
    pub options: Vec<Option<T>>,
    pub selected: usize,
}