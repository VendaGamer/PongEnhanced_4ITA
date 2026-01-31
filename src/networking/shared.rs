use bevy::prelude::*;
use lightyear::prelude::client::ClientPlugins;
use lightyear::prelude::server::ServerPlugins;
use crate::networking::protocol::GameProtocolPlugin;

pub struct GameNetworking;

impl Plugin for GameNetworking {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ServerPlugins::default(),
            ClientPlugins::default(),
            GameProtocolPlugin
        ));
    }
}