#![allow(non_camel_case_types)]

pub mod configs;
pub mod core;
pub mod entities;
pub mod network;
pub mod utils;

pub mod prelude {
    pub use crate::configs::*;
    pub use crate::core::*;
    pub use crate::entities::*;
    pub use crate::network::*;

    pub use crate::utils::logging::*;
    pub use crate::utils::time::*;
    pub use crate::utils::*;
}

pub mod types {
    extern crate nalgebra as na;

    pub type Cash = i64;
    pub type Damage = u8;
    pub type Health = i8;
    pub type RawClientId = u64;
    pub type RawProjectileId = u64;

    pub type RVector2 = raylib::core::math::Vector2;
}
