mod enemy;
mod player;
mod projectile;
mod tile;
mod weapon;

use lib::{
    packets::{Cash, WeaponVariant},
    types::{Color, RVector2, SharedAssets},
    ENTITY_WEAPON_SIZE,
};
use nalgebra::{Rotation, Rotation2, Vector2};
use raylib::prelude::*;
use std::{collections::HashMap, rc::Rc};

pub use enemy::*;
pub use player::*;
pub use projectile::*;
pub use tile::*;
pub use weapon::*;

use crate::game::Assets;

// this bs is shared between the local player (Player) and other dudes (Enemy)
pub struct Invenotry {
    pub cash: Cash,
    pub weapons: HashMap<WeaponVariant, Weapon>,
    selected_weapon: Option<WeaponVariant>,
    assets: SharedAssets<Assets>,
}

impl Invenotry {
    pub fn new(assets: SharedAssets<Assets>) -> Self {
        Self {
            cash: 0,
            weapons: HashMap::new(),
            selected_weapon: None,
            assets,
        }
    }

    pub fn render_weapon(
        &mut self,
        d: &mut RaylibMode2D<RaylibDrawHandle>,
        origin: Vector2<f32>,
        radius: f32,
        orientation: f32,
    ) {
        if let Some(wpn) = &self.selected_weapon() {
            let assets = self.assets.borrow();
            let buffer = assets.textures.get(&wpn.texture).unwrap();

            let (wpn_w, wpn_h) = (
                buffer.width as f32 * ENTITY_WEAPON_SIZE,
                buffer.height as f32 * ENTITY_WEAPON_SIZE,
            );

            let wpn_coords = Self::calculate_point_coords(orientation, radius, origin);
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

                let muzzle = Vector2::new(
                    origin.x + radius + wpn_w * wpn.muzzle.0,
                    origin.y + if flip_y { 1.0 } else { -1.0 } * (wpn_h / 2.0) * wpn.muzzle.1,
                );
                let coords = Rotation2::new(orientation) * (muzzle - origin) + origin;

                d.draw_circle(coords.x as i32, coords.y as i32, 2.0, Color::RED);
            }
        };
    }

    fn calculate_point_coords(orientation: f32, radius: f32, origin: Vector2<f32>) -> (f32, f32) {
        let coords = Vector2::new(radius * orientation.cos(), radius * orientation.sin()) + origin;
        (coords.x, coords.y)
    }

    pub fn selected_weapon(&self) -> Option<&Weapon> {
        if let Some(s_wpn) = &self.selected_weapon {
            self.weapons.get(&s_wpn)
        } else {
            None
        }
    }

    pub fn select(&mut self, variant: WeaponVariant) {
        self.selected_weapon = Some(variant);
    }

    pub fn add(&mut self, variant: WeaponVariant, wpn: Weapon) -> Option<Weapon> {
        self.weapons.insert(variant, wpn)
    }

    pub fn remove(&mut self, variant: WeaponVariant) -> Option<Weapon> {
        self.weapons.remove(&variant)
    }
}
