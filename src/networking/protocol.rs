use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use socket2::{Domain, Protocol, Socket, Type};
use crate::networking::client::{DiscoveredServers, ClientDiscoverySocket};
use crate::models::game::area::LocalPlayerID;
use crate::resources::PlayerAction;
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::input::config::InputConfig;
use lightyear::prelude::input::leafwing;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

pub const UNSPECIFIED: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::BROADCAST, 0);
pub const DISCOVERY_ADDR: SocketAddrV4 =
    SocketAddrV4::new(Ipv4Addr::BROADCAST, DISCOVERY_PORT);
pub const DISCOVERY_PORT: u16 = 6000;

pub const DISCOVERY_CLIENT_MAGIC: &[u8] = b"SENDNUDES";

#[derive(Component, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Reflect, Eq, Hash)]
pub struct RemotePlayerId(pub PeerId, pub LocalPlayerID);

pub struct GameProtocolPlugin;

impl Plugin for GameProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(leafwing::InputPlugin::<PlayerAction> {
            config: InputConfig {
                rebroadcast_inputs: true,
                ..default()
            },
        });

        app.register_component::<Position>()
            .add_prediction()
            .add_should_rollback(position_should_rollback)
            .add_linear_interpolation()
            .add_linear_correction_fn();

        app.register_component::<Rotation>()
            .add_prediction()
            .add_should_rollback(rotation_should_rollback)
            .add_linear_interpolation()
            .add_linear_correction_fn();

        app.register_component::<LinearVelocity>().add_prediction();
        app.register_component::<AngularVelocity>().add_prediction();
    }
}

pub fn make_reusable_udp_socket(port: u16) -> std::io::Result<UdpSocket> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

    socket.set_reuse_address(true)?;
    socket.set_nonblocking(true)?;
    socket.set_broadcast(true)?;
    
    #[cfg(unix)]
    socket.set_reuse_port(true)?;

    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    socket.bind(&SocketAddr::V4(addr).into())?;
    
    Ok(socket.into())
}

#[inline]
fn position_should_rollback(this: &Position, that: &Position) -> bool {
    (this.0 - that.0).length() >= 0.01
}

#[inline]
fn rotation_should_rollback(this: &Rotation, that: &Rotation) -> bool {
    this.angle_between(*that) >= 0.01
}

#[derive(Message, Serialize, Deserialize)]
pub struct ServerAnnouncement;