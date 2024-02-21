mod enemy;
mod player;
mod tile;
mod weapon;

use lib::{
    packets::{Cash, WeaponVariant},
    types::{Color, RVector2, SharedAssets},
    ENTITY_WEAPON_SIZE,
};
use nalgebra::Vector2;
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
        &mut self,
        d: &mut RaylibMode2D<RaylibDrawHandle>,
        assets: SharedAssets<Assets>,
        origin: Vector2<f32>,
        radius: f32,
        orientation: f32,
    ) {
        if let Some(s_wpn) = &self.selected_weapon {
            if let Some(wpn) = self.weapons.get(&s_wpn) {
                let assets = assets.borrow();
                let buffer = assets.textures.get(&wpn.texture).unwrap();

                let (ocos, osin) = (orientation.cos(), orientation.sin());
                let (wpn_w, wpn_h) = (
                    buffer.width as f32 * ENTITY_WEAPON_SIZE,
                    buffer.height as f32 * ENTITY_WEAPON_SIZE,
                );

                let calculate_point_coords = |radius: f32, origin: Vector2<f32>| -> (f32, f32) {
                    let coords = Vector2::new(radius * ocos, radius * osin) + origin;
                    (coords.x, coords.y)
                };

                let wpn_coords = calculate_point_coords(radius, origin);
                let muzzle_coords = calculate_point_coords(radius + wpn_w, origin);
                let theta = orientation.to_degrees();

                let flip_y = if theta.abs() <= 180.0 && theta.abs() > 90.0 {
                    true
                } else {
                    false
                };

                let src_rect = Rectangle::new(
                    0.0,
                    0.0,
                    buffer.width as f32,
                    buffer.height as f32 * if flip_y { -1.0 } else { 1.0 },
                );

                let mzz_x = muzzle_coords.0;
                let mzz_y = muzzle_coords.1;

                let wpn_x = wpn_coords.0;
                let wpn_y = wpn_coords.1;

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
                        d.draw_rectangle_pro(dest_rect, RVector2::zero(), theta, Color::YELLOW);
                    }

                    d.draw_circle(wpn_x as i32, wpn_y as i32, 5.0, Color::YELLOW);
                    d.draw_circle(mzz_x as i32, mzz_y as i32, 5.0, Color::YELLOW);
                }
            }
        };
    }
}
