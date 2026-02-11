use bevy::prelude::*;
use lightyear::prelude::server::{NetcodeConfig, NetcodeServer, ServerPlugins, ServerUdpIo, Start};
use lightyear::prelude::LocalAddr;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use crate::networking::client::DiscoveredServers;
pub const DISCOVERY_PORT: u16 = 25567;
const DISCOVERY_MAGIC: &[u8] = b"ENHANCED_PONG!_SERVER";


pub struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ServerPlugins::default(),
        ));

        app.add_systems(Update, (
            lan_discovery_listener,
        ));
    }
}

pub fn fetch_servers(commands: &mut Commands){
    
}
pub fn lan_discovery_listener(
    mut servers: ResMut<DiscoveredServers>,
) {
    let socket = UdpSocket::bind(("0.0.0.0", DISCOVERY_PORT)).unwrap();
    socket.set_nonblocking(true).unwrap();

    let mut buf = [0u8; 128];

    if let Ok((len, addr)) = socket.recv_from(&mut buf) {
        if &buf[..len] == DISCOVERY_MAGIC {
            if let SocketAddr::V4(v4) = addr {
                if !servers.servers.contains(&v4) {
                    servers.servers.push(v4);
                }
            }
        }
    }
}


pub fn start_server(commands: &mut Commands, server_name: impl Into<String>) {
    let server = commands
        .spawn((
            NetcodeServer::new(NetcodeConfig::default()),
            LocalAddr(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 25566).into()),
            ServerUdpIo::default(),
        ))
        .id();

    commands.trigger(Start { entity: server })
}
