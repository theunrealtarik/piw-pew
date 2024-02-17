pub mod net {
    use std::time::Duration;

    pub const PROTOCOL_ID: u64 = 69;
    pub const DELTA_TIME: Duration = Duration::from_millis(12);
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
    pub enum Tile {
        Wall,
        Flag,
        Empty,
    }
}

pub mod types {
    extern crate nalgebra as na;

    pub type Color = raylib::color::Color;
}

pub mod core {
    use nalgebra::{Point2, Scale2, Vector2};
    use raylib::prelude::*;

    pub trait Update {
        fn update(&mut self, handle: &mut RaylibHandle);
    }

    pub trait Render {
        fn render(&mut self, draw_handle: &mut RaylibDrawHandle);
    }

    pub trait Entity {
        fn get_position(&self) -> &Point2<f32>;
        fn get_health(&self) -> &i8;
        fn get_scale(&self) -> &Scale2<f32>;
        fn get_velocity(&self) -> &Vector2<f32>;
    }
}
