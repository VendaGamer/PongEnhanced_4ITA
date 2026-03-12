use crate::networking::client::GameClientPlugin;
use crate::networking::protocol::GameProtocolPlugin;
use crate::networking::server::GameServerPlugin;
use bevy::prelude::*;
use lightyear::avian2d::plugin::AvianReplicationMode;
use lightyear::avian2d::prelude::LightyearAvianPlugin;

pub struct GameNetworking;

impl Plugin for GameNetworking {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameClientPlugin,
            GameProtocolPlugin,
            GameServerPlugin,
        ));

        app.add_plugins(LightyearAvianPlugin {
            replication_mode: AvianReplicationMode::Transform,
            ..default()
        });
    }
}