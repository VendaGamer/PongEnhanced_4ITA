use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use socket2::{Domain, Protocol, Socket, Type};
use crate::networking::client::{DiscoveredServers, DiscoverySocket};
use crate::networking::server::DISCOVERY_PORT;
use crate::models::game::area::LocalPlayerID;
use crate::resources::PlayerAction;
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::input::config::InputConfig;
use lightyear::prelude::input::leafwing;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

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

        app.insert_resource(DiscoveredServers::default());

        match make_reusable_udp_socket(DISCOVERY_PORT) {
            Ok(socket) => {
                app.insert_resource(DiscoverySocket { socket });
            }
            Err(e) => {
                warn!(
                    "Could not bind discovery listener on port {DISCOVERY_PORT}: {e}. \
                     LAN server discovery will be unavailable."
                );
            }
        }
    }
}

fn make_reusable_udp_socket(port: u16) -> std::io::Result<UdpSocket> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

    socket.set_reuse_address(true)?;
    socket.set_nonblocking(true)?;

    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    socket.bind(&SocketAddr::V4(addr).into())?;

    Ok(socket.into())
}

fn position_should_rollback(this: &Position, that: &Position) -> bool {
    (this.0 - that.0).length() >= 0.01
}

fn rotation_should_rollback(this: &Rotation, that: &Rotation) -> bool {
    this.angle_between(*that) >= 0.01
}

#[derive(Message, Serialize, Deserialize)]
pub struct ServerAnnouncement;