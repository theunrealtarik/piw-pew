mod enemy;
mod player;
mod tile;
mod weapon;

use lib::packets::{Cash, WeaponVariant};
use raylib::drawing::{RaylibDrawHandle, RaylibMode2D};
use std::collections::HashMap;

pub use enemy::*;
pub use player::*;
pub use tile::*;
pub use weapon::*;

// this bs is shared between the local player (Player) and other dudes (Enemy)
pub struct Invenotry {
    pub cash: Cash,
    pub weapons: HashMap<WeaponVariant, Weapon>,
    pub selected_weapon: Option<WeaponVariant>,
}

impl Invenotry {
    pub fn new() -> Self {
        Self {
            cash: 0,
            weapons: HashMap::new(),
            selected_weapon: None,
        }
    }

    pub fn render_weapon(&self, d: RaylibMode2D<RaylibDrawHandle>, (x, y): (f32, f32)) {
        self.selected_weapon.as_ref().map(
            |selected_weapon| {
                if let Some(wpn) = self.weapons.get(selected_weapon) {}
            },
        );
    }
}
