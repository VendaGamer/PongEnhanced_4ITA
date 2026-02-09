use crate::networking::protocol::GameProtocolPlugin;
use crate::networking::server::GameServerPlugin;
use bevy::prelude::*;
use lightyear::prelude::client::ClientPlugins;

pub struct GameNetworking;

impl Plugin for GameNetworking {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ClientPlugins::default(),
            GameProtocolPlugin,
            GameServerPlugin,
        ));
    }
}
