use std::iter::Map;
use bevy::prelude::Entity;
use crate::bundles::Component;

#[derive(Component)]
pub struct Menu{
    pub selectables: Option<Map<Entity, Entity>>,
    pub menu_type: MenuType,
}

impl Menu{
    pub fn new(menu_type: MenuType) -> Self{
        Self{
            selectables: None,
            menu_type
        }
    }

}

pub enum MenuType {
    MainMenu,
    SettingsMenu,
    OfflinePlayMenu,
    OnlinePlayMenu
}