use bevy::app::App;
use bevy::input::keyboard::Key::Control;
use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;
use crate::resources::controls::Controls;

pub struct GameStatePersistencePlugin;

impl Plugin for GameStatePersistencePlugin{
    fn build(&self, app: &mut App) {

        InputMap::new([(Controls::Down, KeyCode::KeyW)]);
    app.add_plugins(AssetPlugin::default());
    }
}