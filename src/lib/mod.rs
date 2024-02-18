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
        NET_WORLD_MAP(HashMap<(usize, usize), super::types::Wall>),
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
    pub enum Wall {
        WALL_SIDE,
        WALL_TOP,
    }

    pub type Walls = std::collections::HashMap<(usize, usize), Wall>;
}

pub mod core {
    use nalgebra::{Point2, Scale2, Vector2};
    use raylib::prelude::*;

    pub trait UpdateHandle {
        fn update(&mut self, handle: &RaylibHandle);
    }

    pub trait RenderHandle {
        fn render(&mut self, draw_handle: &mut RaylibDrawHandle)
        where
            Self: AssetsHandle;
    }

    pub trait NetUpdateHandle {
        type Network;
        fn net_update(&mut self, handle: &RaylibHandle, network: &mut Self::Network);
    }

    pub trait NetRenderHandle {
        type Network;
        fn net_render(&mut self, draw_handle: &mut RaylibDrawHandle, network: &mut Self::Network)
        where
            Self: AssetsHandle;
    }

    pub trait AssetsHandle {
        type GameAssets;
        fn get_assets(&self) -> Self::GameAssets;
    }

    pub trait Entity {
        fn get_position(&self) -> &Point2<f32>;
        fn get_health(&self) -> &i8;
        fn get_scale(&self) -> &Scale2<f32>;
        fn get_velocity(&self) -> &Vector2<f32>;
    }
}

pub mod utils {
    pub fn center<T: Copy + std::ops::Div<Output = T> + From<u32>>(width: T, height: T) -> (T, T) {
        let half: T = T::from(2u32);
        let x = width / half;
        let y = height / half;
        (x, y)
    }
}
