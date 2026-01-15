use std::fs;
use bevy::prelude::*;
use crate::resources::GameSettings;

const SETTINGS_FILE: &str = "settings.json";

pub fn save_settings(settings: Res<GameSettings>) {
    if settings.is_changed() {
        if let Ok(json) = serde_json::to_string_pretty(&*settings) {
            let _ = fs::write(SETTINGS_FILE, json);
        }
    }
}

pub fn load_settings(mut commands: Commands) {
    if let Ok(contents) = fs::read_to_string(SETTINGS_FILE) {
        if let Ok(settings) = serde_json::from_str::<GameSettings>(&contents) {
            commands.insert_resource(settings);
            return;
        }
    }
    commands.insert_resource(GameSettings::default());
}