#![allow(non_camel_case_types)]

pub static WORLD_TILE_SIZE: f32 = 50.0;
pub static PLAYER_TILE_SIZE: f32 = WORLD_TILE_SIZE * 0.8;

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

    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct PlayerData {
        pub _id: u64,
        pub position: (f32, f32),
        pub orientation: f32,
        pub name: String,
        pub hp: u8,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum GameNetworkPacket {
        NET_WORLD_MAP(HashMap<(usize, usize), super::types::Tile>),
        NET_WORLD_PLAYERS(HashMap<u64, PlayerData>),
        NET_PLAYER_JOINED(PlayerData),
        NET_PLAYER_GRID_POSITION(u64, (usize, usize)),
        NET_PLAYER_WORLD_POSITION(u64, (f32, f32)),
        NET_PLAYER_ORIENTATION_ANGLE(u64, usize),
    }
}

pub mod types {
    extern crate nalgebra as na;
    use serde::{Deserialize, Serialize};

    pub type RVector2 = raylib::core::math::Vector2;
    pub type Color = raylib::color::Color;

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
