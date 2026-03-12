use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use socket2::{Domain, Protocol, Socket, Type};
use crate::networking::client::{DiscoveredServers, ClientDiscoverySocket};
use crate::models::game::area::LocalPlayerID;
use crate::resources::{OnlineGameConfig, PlayerAction};
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::input::config::InputConfig;
use lightyear::prelude::input::leafwing;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::game::gameplay::GameMode;
use crate::networking::server::LobbyEntity;

pub const DISCOVERY_ADDR: SocketAddrV4 =
    SocketAddrV4::new(Ipv4Addr::BROADCAST, DISCOVERY_PORT);

pub const DISCOVERY_ADDR_LOCAL: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, DISCOVERY_PORT);
pub const DISCOVERY_PORT: u16 = 6000;
pub const DISCOVERY_CLIENT_MAGIC: &[u8] = b"SEND_NUDESI";
pub const UNSPECIFIED_ADDR: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0));


#[derive(Component, Message, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LobbyConfig {
    pub game_mode: GameMode,
    pub points_to_win: u32,
    pub max_players: u8,
}

impl Default for LobbyConfig {
    fn default() -> Self {
        Self {
            game_mode: GameMode::Classic,
            points_to_win: 10,
            max_players: 4,
        }
    }
}


#[derive(Message, Serialize, Deserialize, Clone, Debug)]
pub struct ChangeLobbySettings {
    pub game_mode: GameMode,
    pub points_to_win: u32,
}

#[derive(Message, Serialize, Deserialize, Clone, Debug)]
pub struct LobbyPlayerList {
    pub players: Vec<(PeerId, String)>
}

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

        app.register_component::<LinearVelocity>()
           .add_prediction();

        app.register_component::<AngularVelocity>()
           .add_prediction();

        app.register_component::<LobbyConfig>();
        app.register_component::<LobbyEntity>();

        app.register_message::<ChangeLobbySettings>()
           .add_direction(NetworkDirection::ClientToServer);

        app.register_message::<LobbyPlayerList>()
           .add_direction(NetworkDirection::ServerToClient);
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