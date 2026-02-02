use crate::networking::protocol::GameProtocolPlugin;
use bevy::prelude::*;
use lightyear::prelude::client::ClientPlugins;
use lightyear::prelude::server::ServerPlugins;

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