use crate::systems::settings::persistence::{load_settings, save_settings};
use bevy::app::App;
use bevy::prelude::*;

pub struct GameStatePersistencePlugin;

impl Plugin for GameStatePersistencePlugin {
    fn build(&self, app: &mut App) { 
        app.add_systems(PreStartup, load_settings)
            .add_systems(Update, save_settings);
    }
}