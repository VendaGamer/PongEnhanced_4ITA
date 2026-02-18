use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use bevy::prelude::*;
use lightyear::prelude::client::ClientPlugins;
use crate::networking::protocol::make_reusable_udp_socket;
use crate::networking::server::{BroadcastTimer, DISCOVERY_PORT, DISCOVERY_SERVER_MAGIC, GAME_PORT};

#[derive(Resource, Default)]
pub struct DiscoveredServers {
    pub servers: Vec<SocketAddrV4>,
}

#[derive(Resource)]
pub struct ClientDiscoverySocket {
    pub socket: UdpSocket,
}

pub struct GameClientPlugin;
impl Plugin for GameClientPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_plugins(ClientPlugins::default());
        
        app.add_systems(Update, (
            lan_discovery_sender,
            lan_discovery_receiver
        ));

        app.insert_resource(DiscoveredServers::default());

        match make_reusable_udp_socket(DISCOVERY_PORT) {
            Ok(socket) => {
                app.insert_resource(ClientDiscoverySocket { socket });
            }
            Err(e) => {
                error!(
                    "Could not bind discovery listener on port {DISCOVERY_PORT}: {e}. \
                     LAN server discovery will be unavailable."
                );
            }
        }
    }
}

pub fn lan_discovery_receiver(
    socket: Res<ClientDiscoverySocket>,
    mut servers: ResMut<DiscoveredServers>,
) {
    let mut buf = [0u8; 256];

    loop {
        match socket.socket.recv_from(&mut buf) {
            Ok((len, SocketAddr::V4(addr))) if &buf[..len] == DISCOVERY_SERVER_MAGIC => {
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

pub fn lan_discovery_sender(
    mut timer: ResMut<BroadcastTimer>,
    socket: Option<Res<ClientDiscoverySocket>>,
    time: Res<Time>,
) {
    let Some(socket) = socket else { return };

    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    send_discovery_message(&socket);
}

#[inline]
pub fn send_discovery_message(disc_soc: &ClientDiscoverySocket) {
    const BROADCAST_ADDR: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::BROADCAST, DISCOVERY_PORT);

    if let Err(e) = disc_soc.socket.send_to(crate::networking::server::DISCOVERY_CLIENT_MAGIC, BROADCAST_ADDR) {
        warn!("Failed to send discovery broadcast: {e}");
    } else {
        info!("Sent discovery broadcast");
    }
}