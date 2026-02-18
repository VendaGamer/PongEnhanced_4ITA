use crate::networking::client::GameClientPlugin;
use crate::networking::protocol::GameProtocolPlugin;
use crate::networking::server::GameServerPlugin;
use bevy::prelude::*;

pub struct GameNetworking;

impl Plugin for GameNetworking {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameClientPlugin,
            GameProtocolPlugin,
            GameServerPlugin,
        ));
    }
}
