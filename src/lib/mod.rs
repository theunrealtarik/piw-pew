#![allow(non_camel_case_types)]

pub mod net {
    use std::time::Duration;

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
    use nalgebra::Vector2;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    pub enum GameNetworkPacket {
        NET_WORLD_MAP(HashMap<(usize, usize), super::types::Tile>),
        NET_PLAYER_POSITION(Vector2<usize>),
        NET_PLAYER_ORIENTATION_ANGLE(usize),
        NET_PLAYER_NAME(String),
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
