use std::net::{SocketAddrV4, UdpSocket};
use crate::bundles::Resource;

#[derive(Resource, Default)]
pub struct DiscoveredServers {
    pub servers: Vec<SocketAddrV4>,
}

#[derive(Resource)]
pub struct DiscoverySocket {
    pub socket: UdpSocket,
}