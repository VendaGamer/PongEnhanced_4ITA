use std::fmt::format;
use bevy::prelude::*;
use lightyear::prelude::server::{NetcodeConfig, NetcodeServer, ServerPlugins, ServerUdpIo, Start};
use lightyear::prelude::*;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use bevy::log::tracing::Instrument;
use lightyear::link::LinkStart;
use lightyear::netcode::client::ClientConfig;
use lightyear::netcode::{Key, NetcodeClient, ServerConfig};
use socket2::{Domain, Protocol, SockAddr, SockAddrStorage, Socket, Type};
use crate::components::ui::ServerList;
use crate::networking::client::{DiscoveredServers, ClientDiscoverySocket};
use crate::networking::protocol::{make_reusable_udp_socket, DISCOVERY_ADDR, DISCOVERY_CLIENT_MAGIC, DISCOVERY_PORT, UNSPECIFIED_ADDR};
use crate::resources::OnlineGameConfig;
const BROADCAST_INTERVAL_SECS: f32 = 30.0;

#[derive(Component)]
pub struct ServerDiscoverySocket {
    pub socket: UdpSocket,
}

#[derive(Resource)]
pub struct BroadcastTimer(pub Timer);

#[derive(Component)]
pub struct ServerName(pub String);

pub struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ServerPlugins::default());

        app.insert_resource(BroadcastTimer(Timer::from_seconds(
            BROADCAST_INTERVAL_SECS,
            TimerMode::Repeating,
        )));

        app.add_systems(Update, lan_discovery_responder);
    }
}

pub fn lan_discovery_responder(
    socket: Single<&ServerDiscoverySocket>,
    server: Single<(&ServerName, &LocalAddr), With<NetcodeServer>>,
) {
    let mut buf = [0u8; 256];

    loop {
        match socket.socket.recv_from(&mut buf) {
            Ok((len, SocketAddr::V4(addr))) => {
                if &buf[..len] == DISCOVERY_CLIENT_MAGIC {
                    info!("Responding to discovery from {}", addr);
                    
                    let server_ip = match UdpSocket::bind("0.0.0.0:0") {
                        Ok(temp) => {
                            if temp.connect(addr).is_ok() {
                                match temp.local_addr() {
                                    Ok(a) => a.ip().to_string(),
                                    Err(_) => "0.0.0.0".to_string(),
                                }
                            } else {
                                "0.0.0.0".to_string()
                            }
                        }
                        Err(_) => "0.0.0.0".to_string(),
                    };
                    
                    let resp = format!(
                        "OKBRO\nNAME {}\nIP {}:{}\n\n\n",
                        server.0 .0,
                        server_ip,
                        server.1 .0.port()
                    );

                    if let Err(e) = socket.socket.send_to(resp.as_bytes(), addr) {
                        warn!("Could not respond to discovery request from {addr}: {e}");
                    } else {
                        info!("Sent discovery response to {addr}");
                    }
                } else {
                    if let Ok(strmsg) = std::str::from_utf8(&buf[..len]) {
                        info!("Received non discovery client magic message: {}", strmsg);
                    }
                }
            }
            _ => {break;}
        }
    }
}

pub fn start_server(
    commands: &mut Commands,
    config: &OnlineGameConfig
) {
    if let Ok(socket) = make_reusable_udp_socket(DISCOVERY_PORT) {

        let mut local_addr: LocalAddr = LocalAddr(UNSPECIFIED_ADDR.into());
        {
            let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
            local_addr = LocalAddr(socket.local_addr().unwrap());
        }

        
        let server = commands.spawn((
                NetcodeServer::new(NetcodeConfig::default()),
                local_addr,
                ServerUdpIo::default(),
                ServerDiscoverySocket { socket: socket.into() },
                ServerName(config.server_name.clone()),
            ))
            .id();
        
        
        commands.trigger(Start { entity: server });
    } else {
        error!("Could not start server");
    }
}