use bevy::prelude::*;
use lightyear::prelude::server::{NetcodeConfig, NetcodeServer, ServerPlugins, ServerUdpIo, Start};
use lightyear::prelude::LocalAddr;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use crate::networking::client::DiscoveredServers;

pub const GAME_PORT: u16 = 25566;
pub const DISCOVERY_PORT: u16 = 25567;
const DISCOVERY_MAGIC: &[u8] = b"ENHANCED_PONG!_SERVER";
const BROADCAST_INTERVAL_SECS: f32 = 2.0;

#[derive(Resource)]
pub struct BroadcastSocket {
    socket: UdpSocket,
}

#[derive(Resource)]
pub struct BroadcastTimer(pub Timer);

pub struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ServerPlugins::default());

        app.insert_resource(BroadcastTimer(Timer::from_seconds(
            BROADCAST_INTERVAL_SECS,
            TimerMode::Repeating,
        )));

        app.add_systems(Update, (
            lan_discovery_receiver,
        ));
    }
}

pub fn lan_discovery_receiver(
    socket: Option<Res<crate::networking::client::DiscoverySocket>>,
    mut servers: ResMut<DiscoveredServers>,
) {
    let Some(socket) = socket else { return };

    let mut buf = [0u8; 256];

    loop {
        match socket.socket.recv_from(&mut buf) {
            Ok((len, SocketAddr::V4(addr))) if &buf[..len] == DISCOVERY_MAGIC => {
                let game_addr = SocketAddrV4::new(*addr.ip(), GAME_PORT);
                if !servers.servers.contains(&game_addr) {
                    info!("Discovered server at {game_addr}");
                    servers.servers.push(game_addr);
                }
            }
            _ => break,
        }
    }
}

pub fn start_server(commands: &mut Commands) {
    let server = commands
        .spawn((
            NetcodeServer::new(NetcodeConfig::default()),
            LocalAddr(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, GAME_PORT).into()),
            ServerUdpIo::default(),
        ))
        .id();

    commands.trigger(Start { entity: server });
}