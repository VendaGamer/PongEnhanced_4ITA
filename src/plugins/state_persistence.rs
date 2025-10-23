use bevy::app::App;
use bevy::prelude::*;

pub struct GameStatePersistencePlugin;

impl Plugin for GameStatePersistencePlugin{
    fn build(&self, app: &mut App) {
    app.add_plugins(AssetPlugin::default());
    }
}