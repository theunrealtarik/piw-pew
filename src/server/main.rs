#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate nalgebra as na;
extern crate rmp_serde as rmps;
extern crate serde;
extern crate serde_derive;

use rand::prelude::*;
use raylib::math::{self, Rectangle};
use rmps::Serializer;
use serde::{Deserialize, Serialize};

use lib::prelude::*;
use lib::types::*;

use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ClientId, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
};

use std::{
    collections::HashMap,
    env::current_dir,
    fs::File,
    io::{self, Read},
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

fn main() {
    env_logger::init_from_env(Logger::env());

    let addr = Ipv4Addr::new(0, 0, 0, 0);
    let port = 6969;

    let public_addr = SocketAddr::new(IpAddr::V4(addr), port);
    let connection_config = ConnectionConfig::default();

    let map = match Map::load("default.map") {
        Ok(map) => {
            log::info!("map loaded successfuly");
            map
        }
        Err(err) => {
            log::error!("{:?}", err);
            std::process::exit(1);
        }
    };

    let map_buffer = map.serialized();

    let mut server: RenetServer = RenetServer::new(connection_config);
    let mut state = ServerState::default();

    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: SERVER_MAX_CLIENTS,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let socket: UdpSocket = UdpSocket::bind(public_addr).unwrap();
    let mut transport = match NetcodeServerTransport::new(server_config, socket) {
        Ok(t) => {
            log::info!("transporting layer is setup");
            t
        }
        Err(_) => {
            log::error!("failed to setup transporting layer");
            std::process::exit(1);
        }
    };

    loop {
        let delta_time = DELTA_TIME;
        server.update(delta_time);
        transport.update(delta_time, &mut server).unwrap();

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    if let Some(user_data) = transport.user_data(client_id) {
                        // get joined player gender identification
                        let stop_index = user_data.iter().position(|&byte| byte == 0).unwrap();
                        let name = String::from_utf8_lossy(&user_data[0..stop_index]).to_string();
                        let rnd_spwn = map.get_random_spawn_position();

                        let player = Client::new(
                            client_id,
                            (
                                rnd_spwn.0 as f32 * WORLD_TILE_SIZE,
                                rnd_spwn.1 as f32 * WORLD_TILE_SIZE,
                            ),
                        );

                        state.players_count += 1;
                        state.players.insert(client_id, player.clone());
                        log::info!(
                            "client connected {} ({}/{})",
                            client_id,
                            state.players_count,
                            transport.max_clients()
                        );

                        // inform joined player
                        server.send_message(
                            client_id,
                            DefaultChannel::ReliableOrdered,
                            map_buffer.clone(),
                        );

                        let enemies_buffer = GameNetworkPacket::NET_WORLD_PLAYERS(
                            state
                                .get_players_raw()
                                .into_iter()
                                .filter(|(id, _)| *id != client_id.raw())
                                .into_iter()
                                .collect(),
                        )
                        .serialized()
                        .unwrap();

                        server.send_message(
                            client_id,
                            DefaultChannel::ReliableOrdered,
                            enemies_buffer,
                        );

                        let rng_buffer = GameNetworkPacket::NET_PLAYER_JOINED(player.data)
                            .serialized()
                            .unwrap();
                        server.broadcast_message(DefaultChannel::ReliableOrdered, rng_buffer);
                    };
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    state.players_count -= 1;
                    state.players.remove(&client_id);
                    server.broadcast_message(
                        DefaultChannel::ReliableUnordered,
                        GameNetworkPacket::NET_PLAYER_LEFT(client_id.raw())
                            .serialized()
                            .unwrap(),
                    );
                    log::warn!(
                        "client disconnected {} ({}/{})",
                        client_id,
                        state.players_count,
                        transport.max_clients()
                    );
                    log::warn!("reason: {}", reason);
                }
            }
        }

        for client_id in server.clients_id() {
            while let Some(message) =
                server.receive_message(client_id, DefaultChannel::ReliableUnordered)
            {
                if let (Ok(packet), Some(player)) = (
                    rmp_serde::from_slice::<GameNetworkPacket>(&message),
                    state.players.get_mut(&client_id),
                ) {
                    match packet {
                        GameNetworkPacket::NET_PROJECTILE_CREATE(projectile) => {
                            state.projectiles.insert(projectile.id, projectile);
                            server.broadcast_message(DefaultChannel::ReliableUnordered, message);
                        }

                        GameNetworkPacket::NET_PLAYER_DIED(id) => {
                            if player.id.raw() == id {
                                let rnd_spwn = map.get_random_spawn_position();

                                player.data.cash -= 500;
                                player.data.cash = nalgebra::clamp(player.data.cash, 0, 16000);
                                player.data.position = (
                                    rnd_spwn.0 as f32 * WORLD_TILE_SIZE,
                                    rnd_spwn.1 as f32 * WORLD_TILE_SIZE,
                                );
                                player.data.weapon = WeaponVariant::AKA_69;

                                let killer_id = player.data._last.clone();
                                player.data._last = None;

                                server.broadcast_message(
                                    DefaultChannel::ReliableUnordered,
                                    GameNetworkPacket::NET_PLAYER_RESPAWN(id, player.data)
                                        .serialized()
                                        .unwrap(),
                                );

                                if let Some(id) = killer_id {
                                    if let Some(player) =
                                        state.players.get_mut(&ClientId::from_raw(id))
                                    {
                                        player.data.cash += 500;
                                        player.data.cash =
                                            nalgebra::clamp(player.data.cash, 0, 16000);

                                        server.send_message(
                                            player.id,
                                            DefaultChannel::ReliableUnordered,
                                            GameNetworkPacket::NET_PLAYER_KILL_REWARD(player.data)
                                                .serialized()
                                                .unwrap(),
                                        );
                                    }
                                }
                            }
                        }
                        GameNetworkPacket::NET_PLAYER_WEAPON(variant) => {
                            let wpn = variant.weapon_instance();
                            let price = *wpn.stats.price() as i64;

                            if player.data.cash >= price {
                                player.data.cash -= price;

                                server.send_message(
                                    client_id,
                                    DefaultChannel::ReliableUnordered,
                                    GameNetworkPacket::NET_PLAYER_WEAPON(variant)
                                        .serialized()
                                        .unwrap(),
                                )
                            }
                        }
                        GameNetworkPacket::NET_PLAYER_WEAPON_SELECT(_, _) => server
                            .broadcast_message_except(
                                client_id,
                                DefaultChannel::ReliableUnordered,
                                message,
                            ),
                        _ => {}
                    }
                }
            }

            while let Some(message) = server.receive_message(client_id, DefaultChannel::Unreliable)
            {
                if let (Ok(packet), Some(player)) = (
                    rmp_serde::from_slice::<GameNetworkPacket>(&message),
                    state.players.get_mut(&client_id),
                ) {
                    match packet {
                        GameNetworkPacket::NET_PLAYER_WORLD_POSITION(_, (x, y)) => {
                            player.data.position = (x, y);
                            server.broadcast_message_except(
                                client_id,
                                DefaultChannel::Unreliable,
                                message,
                            )
                        }
                        GameNetworkPacket::NET_PLAYER_ORIENTATION(_, _) => {
                            server.broadcast_message(DefaultChannel::Unreliable, message)
                        }
                        _ => {}
                    }
                }
            }
        }

        let mut hits: Vec<u64> = Vec::new();

        for (id, projectile) in &mut state.projectiles {
            projectile.position.0 += projectile.velocity.0;
            projectile.position.1 += projectile.velocity.1;

            let (px, py) = projectile.position;
            let (gx, gy) = (
                (px / WORLD_TILE_SIZE).round() as i32,
                (py / WORLD_TILE_SIZE).round() as i32,
            );

            let t_offsets = POINT_OFFSETS
                .into_iter()
                .map(|(dx, dy)| (gx + dx as i32, gy + dy as i32))
                .collect::<Vec<_>>();

            for (x, y) in t_offsets {
                if let Some(t) = map.tiles.get(&(x, y)) {
                    let tile = Rectangle::new(
                        (x as f32) * WORLD_TILE_SIZE,
                        (y as f32) * WORLD_TILE_SIZE,
                        WORLD_TILE_SIZE,
                        WORLD_TILE_SIZE,
                    );

                    if tile.check_collision_circle_rec(
                        math::Vector2::new(px, py),
                        ENTITY_PROJECTILE_RADIUS,
                    ) && *t != TileVariant::GROUND
                    {
                        hits.push(*id);
                        server.broadcast_message(
                            DefaultChannel::ReliableUnordered,
                            GameNetworkPacket::NET_PROJECTILE_IMPACT(*id, None, projectile.damage)
                                .serialized()
                                .unwrap(),
                        );
                        continue;
                    }
                }
            }

            for (_, player) in &mut state.players {
                let (x, y) = (player.data.position.0, player.data.position.1);
                let prect = Rectangle::new(x, y, ENTITY_PLAYER_SIZE, ENTITY_PLAYER_SIZE);

                if prect.check_collision_circle_rec(
                    math::Vector2::new(px, py),
                    ENTITY_PROJECTILE_RADIUS,
                ) {
                    player.data._last = Some(projectile.shooter);
                    hits.push(*id);
                    server.broadcast_message(
                        DefaultChannel::ReliableUnordered,
                        GameNetworkPacket::NET_PROJECTILE_IMPACT(
                            *id,
                            Some(player.id.raw()),
                            projectile.damage,
                        )
                        .serialized()
                        .unwrap(),
                    );
                    continue;
                }
            }
        }

        hits.iter().for_each(|i| {
            state.projectiles.remove(i);
        });

        transport.send_packets(&mut server);
        std::thread::sleep(delta_time);
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Client {
    id: ClientId,
    data: PlayerData,
}

impl Client {
    fn new(id: ClientId, (x, y): (f32, f32)) -> Self {
        Self {
            id,
            data: PlayerData {
                _id: id.raw(),
                _last: None,
                position: (x, y),
                orientation: 0.0,
                health: 100,
                weapon: WeaponVariant::DEAN_1911,
                cash: 200,
            },
        }
    }
}

#[derive(Debug)]
pub struct ServerState {
    players: HashMap<ClientId, Client>,
    players_count: usize,
    projectiles: HashMap<RawProjectileId, ProjectileData>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            players: HashMap::new(),
            projectiles: HashMap::new(),
            players_count: 0,
        }
    }
}

impl ServerState {
    pub fn get_players_raw(&self) -> HashMap<u64, PlayerData> {
        HashMap::from(
            self.players
                .clone()
                .into_iter()
                .map(|(client_id, client)| (client_id.raw(), client.data))
                .collect::<Vec<(u64, PlayerData)>>()
                .into_iter()
                .collect::<HashMap<u64, PlayerData>>(),
        )
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Map {
    pub tiles: Tiles,
}

impl Map {
    fn load(name: &str) -> Result<Self, io::Error> {
        let map_path = current_dir().unwrap().join("maps").join(name);

        log::debug!("{:?}", map_path);

        let mut map: Tiles = HashMap::new();
        match File::open(map_path) {
            Ok(ref mut file) => {
                let mut buffer = String::new();
                match file.read_to_string(&mut buffer) {
                    Ok(_bytes) => {
                        for (y, line) in buffer.lines().enumerate() {
                            for (x, symbol) in line.chars().enumerate() {
                                let tile = match symbol {
                                    'S' => TileVariant::WALL_SIDE,
                                    'T' => TileVariant::WALL_TOP,
                                    _ => TileVariant::GROUND,
                                };
                                map.insert((x as i32, y as i32), tile);
                            }
                        }

                        log::debug!("\n{}", buffer);
                        Ok(Self { tiles: map })
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }

    fn serialized(&self) -> Vec<u8> {
        let packet = GameNetworkPacket::NET_WORLD_MAP(self.tiles.clone());
        let mut map_buffer = Vec::new();

        packet
            .serialize(&mut Serializer::new(&mut map_buffer))
            .unwrap();

        map_buffer
    }

    fn get_ground(&self) -> Tiles {
        let mut map: Tiles = HashMap::new();
        for ((x, y), tile) in &self.tiles {
            if *tile == TileVariant::GROUND {
                map.insert((*x, *y), tile.clone());
            }
        }

        map
    }

    fn get_random_spawn_position(&self) -> (i32, i32) {
        let mut rng = thread_rng();
        let ground_tiles = self.get_ground();
        let ground_tiles = ground_tiles
            .iter()
            .collect::<Vec<(&(i32, i32), &TileVariant)>>();
        let tile_pair = ground_tiles.choose(&mut rng).unwrap();

        *tile_pair.0
    }
    fn bounds(&self) -> (f32, f32) {
        let length = (self.tiles.len() as f32).sqrt() * WORLD_TILE_SIZE;
        (length, length)
    }

    fn in_of_bounds(&self, x: f32, y: f32, width: f32, height: f32) -> bool {
        let bounds = self.bounds();
        x > 0.0 && x <= bounds.0 - width && y > 0.0 && y < bounds.1 - height
    }
}
