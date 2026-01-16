use std::fs;
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow};
use crate::resources::GameSettings;

const SETTINGS_FILE: &str = "settings.json";

pub fn save_settings(settings: Res<GameSettings>) {
    if settings.is_changed() {
        if let Ok(json) = serde_json::to_string_pretty(&*settings) {
            let _ = fs::write(SETTINGS_FILE, json);
        }
    }
}

pub fn load_settings(mut commands: Commands, mut window: Query<&mut Window, With<PrimaryWindow>>) {

    let settings: GameSettings;
    let mut primary_window = window.single_mut().expect("No Primary Window");

    if let Ok(contents) = fs::read_to_string(SETTINGS_FILE) {
        if let Ok(loaded) = serde_json::from_str::<GameSettings>(&contents) {
            settings = loaded;
        }else{
            settings = GameSettings::default();
        }
    }else{
        settings = GameSettings::default();
    }

    primary_window.mode = settings.video_mode.clone();
    commands.insert_resource(settings);
}