mod enemy;
mod player;
mod tile;
mod weapon;

use lib::{
    packets::{Cash, WeaponVariant},
    types::{Color, RVector2, SharedAssets},
    ENTITY_WEAPON_SIZE,
};
use raylib::prelude::*;
use std::collections::HashMap;

pub use enemy::*;
pub use player::*;
pub use tile::*;
pub use weapon::*;

use crate::game::Assets;

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

    pub fn render_weapon(
        &self,
        d: &mut RaylibMode2D<RaylibDrawHandle>,
        assets: SharedAssets<Assets>,
        coords: (f32, f32),
        (flip_x, flip_y): (bool, bool),
        theta: f32,
    ) {
        if let Some(s_wpn) = &self.selected_weapon {
            if let Some(wpn) = self.weapons.get(&s_wpn) {
                let assets = assets.borrow();
                let buffer = assets.textures.get(&wpn.texture).unwrap();

                let src_rect = Rectangle::new(
                    0.0,
                    0.0,
                    buffer.width as f32,
                    buffer.height as f32 * if flip_y { -1.0 } else { 1.0 },
                );

                let (wpn_w, wpn_h) = (
                    buffer.width as f32 * ENTITY_WEAPON_SIZE,
                    buffer.height as f32 * ENTITY_WEAPON_SIZE,
                );

                let wpn_x = coords.0;
                let wpn_y = coords.1;

                let dest_rect = Rectangle::new(wpn_x, wpn_y, wpn_w, wpn_h);

                d.draw_texture_pro(
                    buffer,
                    src_rect,
                    dest_rect,
                    RVector2::new(0.0, wpn_h / 2.0),
                    theta,
                    Color::WHITE,
                );

                #[cfg(debug_assertions)]
                {
                    if d.is_key_down(KeyboardKey::KEY_LEFT_ALT) {
                        d.draw_rectangle_pro(
                            Rectangle::new(wpn_x, wpn_y, wpn_w, wpn_h),
                            RVector2::zero(),
                            theta,
                            Color::YELLOW,
                        );
                    }
                }
            }
        };
    }
}
