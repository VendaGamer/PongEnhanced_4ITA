use crate::bundles::{AppExit, Commands, Entity, MessageWriter, On, Query, With};
use crate::bundles::offline_menu::OfflinePlayMenuBundle;
use crate::bundles::online_menu::OnlinePlayMenuBundle;
use crate::bundles::settings_menu::SettingsMenuBundle;
use crate::components::ui::MainMenu;
use crate::systems::ButtonPressed;

pub fn on_offline(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    main_menu: Query<Entity, With<MainMenu>>,
) {
    for entity in &main_menu {
        commands.entity(entity).despawn();
    }
    OfflinePlayMenuBundle::spawn_offline_menu(&mut commands);
}

pub fn on_online(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    main_menu: Query<Entity, With<MainMenu>>,
) {
    for entity in &main_menu {
        commands.entity(entity).despawn();
    }
    OnlinePlayMenuBundle::spawn_online_menu(&mut commands);
}

pub fn on_settings(
    _press: On<ButtonPressed>,
    mut commands: Commands,
    main_menu: Query<Entity, With<MainMenu>>,
) {
    for entity in &main_menu {
        commands.entity(entity).despawn();
    }
    SettingsMenuBundle::spawn_settings_menu(&mut commands);
}

pub fn on_exit(press: On<ButtonPressed>, mut exit: MessageWriter<AppExit>){
    exit.write(AppExit::Success);
}