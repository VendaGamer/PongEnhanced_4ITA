use crate::bundles::Component;

#[derive(Component)]
pub struct Menu{
    pub menu_type: MenuType,
}

impl Menu{
    pub fn new(menu_type: MenuType) -> Self{
        Self{
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