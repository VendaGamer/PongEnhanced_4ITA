use bevy::prelude::*;

pub trait SelectableResource: Send + Sync + 'static {
    fn get_options() -> Vec<(&'static str, Self)>
    where
        Self: Sized;
    fn get_label(&self) -> &'static str;
}
