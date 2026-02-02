use crate::resources::GameSettings;
use bevy::prelude::*;
use std::fs;

const SETTINGS_FILE: &str = "settings.json";

pub fn save_settings(settings: &Res<GameSettings>) {
    if settings.is_changed() {
        if let Ok(json) = serde_json::to_string_pretty(settings.as_ref()) {
            let _ = fs::write(SETTINGS_FILE, json);
        }
    }
}

pub fn load_settings() -> GameSettings {
    let settings: GameSettings;

    if let Ok(contents) = fs::read_to_string(SETTINGS_FILE) {
        if let Ok(loaded) = serde_json::from_str::<GameSettings>(&contents) {
            settings = loaded;
        } else {
            settings = GameSettings::default();
        }
    } else {
        settings = GameSettings::default();
    }

    settings
}
