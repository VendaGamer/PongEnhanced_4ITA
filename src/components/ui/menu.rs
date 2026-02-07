use crate::bundles::Component;

#[derive(Component)]
pub struct SettingsMenu;
#[derive(Component)]
pub struct MainMenu;
#[derive(Component)]
pub struct OfflinePlayMenu;
#[derive(Component)]
pub struct OnlinePlayMenu;
#[derive(Component)]
pub struct PauseMenu;
#[derive(Component)]
pub struct PlayerJoinInMenu(pub u8);
#[derive(Component)]
pub struct OnlineCreateMenu;
#[derive(Component)]
pub struct Menu;
