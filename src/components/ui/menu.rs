use crate::bundles::Component;

#[derive(Component)]
pub enum Menu {
    MainMenu,
    SettingsMenu,
    OfflinePlayMenu,
    OnlinePlayMenu
}