#![allow(non_camel_case_types)]

pub static WORLD_TILE_SIZE: f32 = 100.0;
pub static ENTITY_PLAYER_SIZE: f32 = WORLD_TILE_SIZE * 0.8;
pub static ENTITY_WEAPON_SIZE: f32 = ENTITY_PLAYER_SIZE * 0.003;

pub mod net {
    use std::time::Duration;

    pub const SERVER_MAX_CLIENTS: usize = 12;
    pub const PROTOCOL_ID: u64 = 69;
    pub const DELTA_TIME: Duration = Duration::from_millis(16);
}

pub mod logging {
    use env_logger::{self, Env};

    pub struct Logger;
    impl Logger {
        pub fn env() -> Env<'static> {
            let env = Env::default()
                .filter_or("RUST_LOG", "server=trace,client=trace,lib=trace")
                .write_style_or("RUST_STYLE_LOG", "always");
            env
        }
    }
}

pub mod packets {
    extern crate rmp_serde as rmps;

    use rmps::Serializer;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct PlayerData {
        pub _id: u64,
        pub position: (f32, f32),
        pub orientation: f32,
        pub name: String,
        pub weapon: WeaponVariant,
        pub hp: u8,
    }

    pub type Cash = u64;
    pub type RawClientId = u64;

    #[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Deserialize, Serialize)]
    pub enum WeaponVariant {
        DEAN_1911,
        AKA_69,
        SHOTPEW,
        PRRR,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum GameNetworkPacket {
        NET_WORLD_MAP(HashMap<(usize, usize), super::types::Tile>),
        NET_WORLD_PLAYERS(HashMap<u64, PlayerData>),
        NET_PLAYER_JOINED(PlayerData),
        NET_PLAYER_GRID_POSITION(RawClientId, (usize, usize)),
        NET_PLAYER_WORLD_POSITION(RawClientId, (f32, f32)),
        NET_PLAYER_ORIENTATION_ANGLE(u64, usize),
        NET_PLAYER_LEFT(RawClientId),
        NET_PLAYER_WEAPON_REQUEST(Cash, WeaponVariant),
        NET_PLAYER_WEAPON_RESPONSE(RawClientId, WeaponVariant),
    }

    impl GameNetworkPacket {
        pub fn serialized(&self) -> Result<Vec<u8>, String> {
            let mut buffer: Vec<u8> = Vec::new();
            if let Ok(packet) = self.serialize(&mut Serializer::new(&mut buffer)) {
                return Ok(buffer);
            }

            Err(String::from("failed to serialize packet object"))
        }
    }
}

pub mod types {
    extern crate nalgebra as na;
    use std::{cell::RefCell, rc::Rc};

    use serde::{Deserialize, Serialize};

    pub type RVector2 = raylib::core::math::Vector2;
    pub type Color = raylib::color::Color;
    pub type SharedAssets<T> = Rc<RefCell<T>>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
    pub enum Tile {
        WALL_SIDE,
        WALL_TOP,
        GROUND,
    }

    pub type Tiles = std::collections::HashMap<(usize, usize), Tile>;
}

pub mod utils {
    pub fn center<T: Copy + std::ops::Div<Output = T> + From<u32>>(width: T, height: T) -> (T, T) {
        let half: T = T::from(2u32);
        let x = width / half;
        let y = height / half;
        (x, y)
    }
}
