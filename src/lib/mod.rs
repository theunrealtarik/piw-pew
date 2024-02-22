#![allow(non_camel_case_types)]

use types::Health;

pub static WORLD_TILE_SIZE: f32 = 100.0;
pub static ENTITY_PLAYER_SIZE: f32 = WORLD_TILE_SIZE * 0.8;
pub static ENTITY_WEAPON_SIZE: f32 = ENTITY_PLAYER_SIZE * 0.0018;
pub static ENTITY_PLAYER_MAX_HEALTH: Health = 100;
pub static ENTITY_PROJECTILE_SPEED: u32 = 5; // speed is the abs of velocity, it's not velocity (death threat for every unity tutorial).
pub static ENTITY_PROJECTILE_RADIUS: f32 = 2.0;

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

    use crate::types::{Cash, Damage, Health, RawClientId, RawProjectileId, WeaponVariant};

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct PlayerData {
        pub _id: RawClientId,
        pub position: (f32, f32),
        pub orientation: f32,
        pub name: String,
        pub weapon: WeaponVariant,
        pub hp: Health,
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
        NET_WORLD_MAP(HashMap<(i32, i32), super::types::Tile>),
        NET_WORLD_PLAYERS(HashMap<u64, PlayerData>),
        NET_PLAYER_JOINED(PlayerData),
        NET_PLAYER_GRID_POSITION(RawClientId, (i32, i32)),
        NET_PLAYER_WORLD_POSITION(RawClientId, (f32, f32)),
        NET_PLAYER_ORIENTATION(u64, f32),
        NET_PLAYER_LEFT(RawClientId),
        NET_PLAYER_WEAPON_REQUEST(Cash, WeaponVariant),
        NET_PLAYER_WEAPON_RESPONSE(RawClientId, WeaponVariant),
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
}

pub mod types {
    extern crate nalgebra as na;
    use std::{cell::RefCell, rc::Rc};

    use serde::{Deserialize, Serialize};

    pub type Cash = u64;
    pub type Damage = u8;
    pub type Health = i8;
    pub type RawClientId = u64;
    pub type RawProjectileId = u64;

    pub type RVector2 = raylib::core::math::Vector2;
    pub type Color = raylib::color::Color;
    pub type SharedAssets<T> = Rc<RefCell<T>>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
    pub enum Tile {
        WALL_SIDE,
        WALL_TOP,
        GROUND,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Deserialize, Serialize)]
    pub enum WeaponVariant {
        DEAN_1911,
        AKA_69,
        SHOTPEW,
        PRRR,
    }

    pub type Tiles = std::collections::HashMap<(i32, i32), Tile>;
}

pub mod utils {
    pub fn center<T: Copy + std::ops::Div<Output = T> + From<u32>>(width: T, height: T) -> (T, T) {
        let half: T = T::from(2u32);
        let x = width / half;
        let y = height / half;
        (x, y)
    }

    pub static POINT_OFFSETS: [(i8, i8); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
}
