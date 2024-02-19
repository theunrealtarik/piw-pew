#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate rmp_serde as rmps;
extern crate serde;
extern crate serde_derive;

use lib::WORLD_TILE_SIZE;
use rand::prelude::*;
use rmps::Serializer;
use serde::{Deserialize, Serialize};

use lib::logging::Logger;
use lib::net::{DELTA_TIME, PROTOCOL_ID};
use lib::packets::{GameNetworkPacket, PlayerData};
use lib::types::{Tile, Tiles};

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

#[derive(Debug, Clone)]
pub struct PlayerClient {
    id: ClientId,
    data: PlayerData,
}

impl PlayerClient {
    fn new(id: ClientId, name: String, (x, y): (f32, f32)) -> Self {
        Self {
            id,
            data: PlayerData {
                position: (x, y),
                orientation: 0.0,
                name,
            },
        }
    }
}

#[derive(Debug)]
pub struct GameState {}

pub struct ServerState {
    players: HashMap<ClientId, PlayerClient>,
    players_count: usize,
}

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
    let mut state = ServerState {
        players_count: 0,
        players: HashMap::new(),
    };

    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 64,
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

    log::info!("waiting for connections");
    loop {
        let delta_time = DELTA_TIME;
        server.update(delta_time);
        transport.update(delta_time, &mut server).unwrap();

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    if let Some(user_data) = transport.user_data(client_id) {
                        let stop_index = user_data.iter().position(|&byte| byte == 0).unwrap();
                        let name = String::from_utf8_lossy(&user_data[0..stop_index]).to_string();
                        let rnd_spwn = map.get_random_spawn_position();

                        let player = PlayerClient::new(
                            client_id,
                            name.to_string(),
                            (rnd_spwn.0 as f32, rnd_spwn.1 as f32),
                        );

                        state.players_count += 1;
                        state.players.insert(client_id, player.clone());
                        log::info!(
                            "client connected {} ({}/{})",
                            client_id,
                            state.players_count,
                            transport.max_clients()
                        );

                        server.send_message(
                            client_id,
                            DefaultChannel::ReliableOrdered,
                            map_buffer.clone(),
                        );

                        let mut rng_spwn = Vec::new();
                        GameNetworkPacket::NET_PLAYER_JOINED(player.data)
                            .serialize(&mut Serializer::new(&mut rng_spwn))
                            .unwrap();
                        server.broadcast_message(DefaultChannel::ReliableOrdered, rng_spwn);
                    };
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    state.players_count -= 1;
                    state.players.remove(&client_id);
                    log::info!(
                        "client connected {} ({}/{})",
                        client_id,
                        state.players_count,
                        transport.max_clients()
                    );
                    log::warn!("reason: {}", reason);
                }
            }
        }

        transport.send_packets(&mut server);
        std::thread::sleep(delta_time);
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
                                    'S' => Tile::WALL_SIDE,
                                    'T' => Tile::WALL_TOP,
                                    _ => Tile::GROUND,
                                };
                                map.insert((x, y), tile);
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
            if *tile == Tile::GROUND {
                map.insert((*x, *y), tile.clone());
            }
        }

        map
    }

    fn get_random_spawn_position(&self) -> (usize, usize) {
        let mut rng = thread_rng();
        let ground_tiles = self.get_ground();
        let ground_tiles = ground_tiles
            .iter()
            .collect::<Vec<(&(usize, usize), &Tile)>>();
        let tile_pair = ground_tiles.choose(&mut rng).unwrap();

        *tile_pair.0
    }
}
