use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::io::BufRead;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::str::FromStr;
use bevy::prelude::*;
use lightyear::prelude::client::ClientPlugins;
use socket2::{Domain, Protocol, Socket, Type};
use crate::networking::protocol::{make_reusable_udp_socket, UNSPECIFIED, DISCOVERY_ADDR, DISCOVERY_CLIENT_MAGIC};
use crate::networking::server::{BroadcastTimer};

#[derive(Resource, Default, Deref)]
pub struct DiscoveredServers {
    pub servers: HashSet<DiscoveredServer>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiscoveredServer {
    pub address: SocketAddrV4,
    pub name: String
}

impl Hash for DiscoveredServer {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
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

        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP));

        match socket {
            Ok(socket) => {
                _ = socket.set_nonblocking(true);
                _ = socket.set_broadcast(true);

                app.insert_resource(ClientDiscoverySocket { socket: socket.into() });
            }
            Err(e) => {
                error!(
                    "Could not bind client discovery socket: {e}. \
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
            Ok((len, SocketAddr::V4(addr))) => {
                let mut read = &buf[..(len - 3)]; // - 3 cause 3 new lines at the end

                let map: HashMap<String, String> = read
                    .split(|b| *b == b'\n')
                    .filter_map(|line| {
                        let mut parts = line.splitn(2, |b| *b == b' ');
                        Some((
                            String::from_utf8_lossy(parts.next()?).into_owned(),
                            String::from_utf8_lossy(parts.next()?).into_owned(),
                        ))
                    })
                    .collect();

                if let Some(name) = map.get("NAME"){
                    if let Some(addr) = map.get("IP"){
                        if let Ok(addr) = SocketAddrV4::from_str(addr.as_str()) {
                            servers.servers.insert(DiscoveredServer {
                                name: name.clone(),
                                address: addr,
                            });
                        }
                    }
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
    if let Err(e) = disc_soc.socket.send_to(DISCOVERY_CLIENT_MAGIC, DISCOVERY_ADDR) {
        warn!("Failed to send discovery broadcast: {e}");
    } else {
        info!("Sent discovery broadcast");
    }
}