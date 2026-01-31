use crate::resources::PlayerAction;
use bevy::prelude::*;
use lightyear::prelude::input::leafwing::InputPlugin;
use lightyear::prelude::server::{NetcodeConfig, NetcodeServer, ServerUdpIo, Start};
use lightyear::prelude::LocalAddr;
use std::net::{Ipv4Addr, SocketAddrV4};

pub struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputPlugin::<PlayerAction>::default(),
        ));
    }
}

pub fn start_server(mut commands: Commands) {
    let server = commands
        .spawn((
            NetcodeServer::new(NetcodeConfig::default()),
            LocalAddr(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 25566).into()),
            ServerUdpIo::default(),
        ))
        .id();

    commands.trigger(Start{
        entity: server
    })
}