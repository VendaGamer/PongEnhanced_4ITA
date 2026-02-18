use bevy::prelude::*;
use lightyear::prelude::server::{NetcodeConfig, NetcodeServer, ServerPlugins, ServerUdpIo, Start};
use lightyear::prelude::LocalAddr;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use socket2::{Domain, Protocol, Socket, Type};
use crate::networking::client::{DiscoveredServers, ClientDiscoverySocket};
use crate::networking::protocol::make_reusable_udp_socket;

pub const GAME_PORT: u16 = 25566;
pub const DISCOVERY_PORT: u16 = 25567;

pub const DISCOVERY_CLIENT_MAGIC: &[u8] = b"ENHANCED_PONG!_CLIENT";
pub const DISCOVERY_SERVER_MAGIC: &[u8] = b"ENHANCED_PONG!_SERVER";
const BROADCAST_INTERVAL_SECS: f32 = 30.0;

#[derive(Component)]
pub struct ServerDiscoverySocket {
    pub socket: UdpSocket,
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
            lan_discovery_responder
        ));
    }
}

pub fn lan_discovery_responder(socket: Single<&ServerDiscoverySocket>) {
    let mut buf = [0u8; 256];
    
    loop {
        match socket.socket.recv_from(&mut buf) {
            Ok((len, SocketAddr::V4(addr))) => {

                if &buf[..len] == DISCOVERY_CLIENT_MAGIC {
                    info!("Responding to discovery");

                    if let Err(e) = socket.socket.send_to(DISCOVERY_SERVER_MAGIC, addr) {
                        warn!("Could not respond to discovery request from {addr}: {e}");
                    } else {
                        info!("Sent discovery response to {addr}");
                    }
                } else {
                    let message = std::str::from_utf8(&buf);

                    if let Ok(str) = message {
                        info!("Received non discovery client magic message: {str}")
                    }
                }
                
            },
            _ => break,
        }
    }
}

pub fn start_server(commands: &mut Commands) {
    
    if let Ok(socket) = make_reusable_udp_socket(DISCOVERY_PORT) {
        let server = commands
            .spawn((
                NetcodeServer::new(NetcodeConfig::default()),
                LocalAddr(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, GAME_PORT).into()),
                ServerUdpIo::default(),
                ServerDiscoverySocket { socket }
            ))
            .id();

        commands.trigger(Start { entity: server });
    } else { 
        error!("Could not start server");
    }
}