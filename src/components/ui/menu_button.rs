use bevy::prelude::Component;

#[derive(Component)]
pub enum MenuButton {
    OfflinePlay,
    OnlinePlay,
    Settings,
    ExitGame,
}