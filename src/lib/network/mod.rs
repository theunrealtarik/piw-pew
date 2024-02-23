extern crate rmp_serde as rmps;

pub use renet::transport::ClientAuthentication;
pub use renet::transport::NetcodeClientTransport;
pub use renet::transport::NetcodeError;
pub use renet::ClientId;
pub use renet::ConnectionConfig;
pub use renet::DefaultChannel;
pub use renet::RenetClient;

use rmps::Serializer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::net::UdpSocket;

use crate::prelude::*;
use crate::types::*;
use crate::utils;

pub const SERVER_MAX_CLIENTS: usize = 12;
pub const PROTOCOL_ID: u64 = 69;
pub const DELTA_TIME: Duration = Duration::from_millis(16);

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct PlayerData {
    /// raw player id
    pub _id: RawClientId,
    /// last one who hit the player
    pub _last: Option<RawClientId>,
    pub position: (f32, f32),
    pub orientation: f32,
    pub weapon: WeaponVariant,
    pub health: Health,
    pub cash: Cash,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ProjectileData {
    pub id: RawProjectileId,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub grid: (i32, i32),
    pub orientation: f32,
    pub shooter: RawClientId,
    pub damage: u8,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum GameNetworkPacket {
    NET_WORLD_MAP(HashMap<(i32, i32), TileVariant>),
    NET_WORLD_PLAYERS(HashMap<u64, PlayerData>),
    NET_PLAYER_JOINED(PlayerData),
    NET_PLAYER_DIED(RawClientId),
    NET_PLAYER_RESPAWN(RawClientId, PlayerData),
    NET_PLAYER_KILL_REWARD(PlayerData),
    NET_PLAYER_GRID_POSITION(RawClientId, (i32, i32)),
    NET_PLAYER_WORLD_POSITION(RawClientId, (f32, f32)),
    NET_PLAYER_ORIENTATION(u64, f32),
    NET_PLAYER_LEFT(RawClientId),
    NET_PLAYER_WEAPON_REQUEST(Cash, WeaponVariant),
    NET_PLAYER_WEAPON_RESPONSE(WeaponVariant),
    NET_PROJECTILE_CREATE(ProjectileData),
    NET_PROJECTILE_IMPACT(RawProjectileId, Option<RawClientId>, Damage),
}

impl GameNetworkPacket {
    pub fn serialized(&self) -> Result<Vec<u8>, String> {
        let mut buffer: Vec<u8> = Vec::new();
        match self.serialize(&mut Serializer::new(&mut buffer)) {
            Ok(_) => Ok(buffer),
            Err(_) => Err(String::from("failed to serialize packet object")),
        }
    }
}

pub struct GameNetwork {
    pub client: RenetClient,
    pub transport: NetcodeClientTransport,
    pub current_time: Duration,
    pub authentication: ClientAuthentication,
    pub uuid: u64,
    pub protocol_id: u64,
}

impl GameNetwork {
    pub fn connect(
        server_addr: SocketAddr,
        current_time: Duration,
        protocol_id: u64,
        data: [u8; 256],
    ) -> Result<Self, NetcodeError> {
        let uuid = utils::raw_uuid();

        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let client = RenetClient::new(ConnectionConfig::default());

        let authentication = ClientAuthentication::Unsecure {
            server_addr,
            client_id: uuid,
            user_data: Some(data),
            protocol_id,
        };

        match NetcodeClientTransport::new(current_time, authentication.clone(), socket) {
            Ok(transport) => Ok(Self {
                client,
                transport,
                current_time,
                authentication,
                uuid,
                protocol_id,
            }),
            Err(err) => Err(err),
        }
    }
}
