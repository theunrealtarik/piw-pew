#![allow(non_camel_case_types)]

use std::time::Duration;

use lazy_static::lazy_static;
use lib::types::WeaponVariant;

use nalgebra::{Rotation2, Vector, Vector2};
use raylib::prelude::*;
use std::collections::HashMap;

use lib::{
    types::{Cash, Color, RVector2, SharedAssets},
    ENTITY_WEAPON_SIZE,
};

use crate::game::{Assets, TEXTURE};

#[derive(Debug, Clone)]
pub enum WeaponAccuracy {
    Low,
    Moderate,
    High,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct WeaponStats {
    name: &'static str,
    damage: u8,
    accuracy: WeaponAccuracy,
    reload_time: Duration,
    fire_time: Duration,
    pub mag_size: u8,
    pub total_ammo: u8,
    price: u32,
}

impl WeaponStats {
    pub fn new(
        name: &'static str,
        damage: u8,
        accuracy: WeaponAccuracy,
        fire_time: Duration,
        reload_time: Duration,
        mag_size: u8,
        mags: u8,
        price: u32,
    ) -> Self {
        Self {
            name,
            damage,
            accuracy,
            reload_time,
            fire_time,
            mag_size,
            total_ammo: mag_size * mags,
            price,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn damage(&self) -> &u8 {
        &self.damage
    }

    pub fn price(&self) -> &u32 {
        &self.price
    }

    pub fn accuracy(&self) -> &WeaponAccuracy {
        &self.accuracy
    }

    pub fn reload_time(&self) -> &Duration {
        &self.reload_time
    }

    pub fn fire_time(&self) -> &Duration {
        &self.fire_time
    }
}

// weapons stats
lazy_static! {
    static ref WPN_STATS_AKA_69: WeaponStats = WeaponStats::new(
        "AKA-69",
        40,
        WeaponAccuracy::Moderate,
        Duration::from_millis(100),
        Duration::from_millis(1500),
        30,
        4,
        2700
    );
    static ref WPN_STATS_SHOTPEW: WeaponStats = WeaponStats::new(
        "PUMP Shotpew",
        25,
        WeaponAccuracy::Low,
        Duration::from_millis(300),
        Duration::from_millis(2000),
        5,
        5,
        2100
    );
    static ref WPN_STATS_DEAN_1911: WeaponStats = WeaponStats::new(
        "DEAN 1911",
        25,
        WeaponAccuracy::High,
        Duration::from_millis(1100),
        Duration::from_millis(1100),
        7,
        4,
        400
    );
    static ref WPN_STATS_PRRR: WeaponStats = WeaponStats::new(
        "PRRR",
        45,
        WeaponAccuracy::Low,
        Duration::from_millis(50),
        Duration::from_millis(2500),
        30,
        4,
        5200
    );
}

macro_rules! wpn_stats_mapping {
    ($($field:ident),*) => {
        #[derive(Debug)]
        pub enum WeaponStatsMapping {
            $($field),*
        }

        impl WeaponStatsMapping {
            pub fn get(&self) -> &WeaponStats {
                match self {
                    $(WeaponStatsMapping::$field => &$field),*
                }
            }
        }
    };
}

wpn_stats_mapping!(
    WPN_STATS_AKA_69,
    WPN_STATS_SHOTPEW,
    WPN_STATS_DEAN_1911,
    WPN_STATS_PRRR
);

#[derive(Debug)]
pub struct Weapon {
    pub variant: WeaponVariant,
    pub texture: TEXTURE,
    pub muzzle: (f32, f32),
    pub stats: &'static WeaponStats,
    pub curr_total_ammo: u8,
    pub curr_mag_ammo: u8,
}

impl Weapon {
    pub fn new(variant: WeaponVariant) -> Self {
        let _ = WeaponStatsMapping::WPN_STATS_AKA_69.get();

        match variant {
            WeaponVariant::DEAN_1911 => {
                let stats = WeaponStatsMapping::WPN_STATS_DEAN_1911.get();
                Weapon {
                    variant,
                    texture: TEXTURE::WPN_DEAN,
                    muzzle: (0.942, 0.685),
                    stats,
                    curr_total_ammo: stats.total_ammo,
                    curr_mag_ammo: stats.mag_size,
                }
            }
            WeaponVariant::AKA_69 => {
                let stats = WeaponStatsMapping::WPN_STATS_DEAN_1911.get();
                Weapon {
                    variant,
                    texture: TEXTURE::WPN_AKA,
                    muzzle: (0.988, 0.173),
                    stats: WeaponStatsMapping::WPN_STATS_AKA_69.get(),
                    curr_total_ammo: stats.total_ammo,
                    curr_mag_ammo: stats.mag_size,
                }
            }
            WeaponVariant::SHOTPEW => {
                let stats = WeaponStatsMapping::WPN_STATS_DEAN_1911.get();
                Weapon {
                    variant,
                    texture: TEXTURE::WPN_SHOTPEW,
                    muzzle: (0.988, 0.046),
                    stats: WeaponStatsMapping::WPN_STATS_SHOTPEW.get(),
                    curr_total_ammo: stats.total_ammo,
                    curr_mag_ammo: stats.mag_size,
                }
            }
            WeaponVariant::PRRR => {
                let stats = WeaponStatsMapping::WPN_STATS_DEAN_1911.get();
                Weapon {
                    variant,
                    texture: TEXTURE::WPN_PRRR,
                    muzzle: (0.988, 0.372),
                    stats: WeaponStatsMapping::WPN_STATS_PRRR.get(),
                    curr_total_ammo: stats.total_ammo,
                    curr_mag_ammo: stats.mag_size,
                }
            }
        }
    }

    pub fn remaining_ammo(&self) -> u8 {
        let remaining = self
            .curr_total_ammo
            .checked_sub(self.curr_mag_ammo)
            .unwrap_or(0);

        remaining
    }

    pub fn reload(&mut self) {
        let delta_ammo = self.stats.mag_size - self.curr_mag_ammo;
        if let Some(total) = self.curr_total_ammo.checked_sub(delta_ammo) {
            self.curr_total_ammo = total;
            self.curr_mag_ammo += delta_ammo;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.curr_mag_ammo == 0
    }

    /// returns the muzzle position in world coords
    pub fn muzzle(
        &self,
        buffer: &Texture2D,
        player_rect: &Rectangle,
        player_orientation: f32,
    ) -> Vector2<f32> {
        let origin = Vector2::new(
            player_rect.x + player_rect.width / 2.0,
            player_rect.y + player_rect.height / 2.0,
        );

        let theta = player_orientation.to_degrees();

        let flip_y = if theta.abs() <= 180.0 && theta.abs() > 90.0 {
            true
        } else {
            false
        };

        let (wpn_w, wpn_h) = (
            buffer.width as f32 * ENTITY_WEAPON_SIZE,
            buffer.height as f32 * ENTITY_WEAPON_SIZE,
        );

        let muzzle = Vector2::new(
            origin.x + player_rect.width / 2.0 + wpn_w * self.muzzle.0,
            origin.y + if flip_y { 1.0 } else { -1.0 } * (wpn_h / 2.0) * self.muzzle.1,
        );

        let coords = Rotation2::new(player_orientation) * (muzzle - origin) + origin;

        coords
    }
}

/// controllers some puppet's (`Player`, `Enemy`) weapons 'n' ammo
#[derive(Debug)]
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

    pub fn reset_weapons(&mut self) {
        self.weapons = HashMap::new();
        self.add(Weapon::new(WeaponVariant::DEAN_1911));
    }

    pub fn has(&self, variant: &WeaponVariant) -> bool {
        match self.weapons.get(variant) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get(&self, variant: &WeaponVariant) -> Option<&Weapon> {
        self.weapons.get(variant)
    }

    pub fn render_weapon(
        &mut self,
        d: &mut RaylibMode2D<RaylibDrawHandle>,
        player_rect: &Rectangle,
        orientation: f32,
    ) {
        if let Some(wpn) = &self.selected_weapon() {
            let assets = self.assets.borrow();
            let buffer = assets.textures.get(&wpn.texture).unwrap();

            let (wpn_w, wpn_h) = (
                buffer.width as f32 * ENTITY_WEAPON_SIZE,
                buffer.height as f32 * ENTITY_WEAPON_SIZE,
            );

            let radius = player_rect.width / 2.0;
            let origin = Vector2::new(player_rect.x, player_rect.y).add_scalar(radius);

            let wpn_coords =
                Vector2::new(radius * orientation.cos(), radius * orientation.sin()) + origin;
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

            let wpn_x = wpn_coords.x;
            let wpn_y = wpn_coords.y;

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

                let coords = wpn.muzzle(buffer, player_rect, orientation);

                d.draw_circle(coords.x as i32, coords.y as i32, 2.0, Color::RED);
                d.draw_circle_lines(origin.x as i32, origin.y as i32, radius, Color::RED);
            }
        };
    }

    pub fn selected_weapon(&self) -> Option<&Weapon> {
        self.selected_weapon
            .as_ref()
            .and_then(|s_wpn| self.weapons.get(s_wpn))
    }

    pub fn selected_weapon_mut(&mut self) -> Option<&mut Weapon> {
        self.selected_weapon
            .as_ref()
            .and_then(|s_wpn| self.weapons.get_mut(s_wpn))
    }

    pub fn select(&mut self, variant: WeaponVariant) {
        self.selected_weapon = Some(variant);
    }

    pub fn add(&mut self, wpn: Weapon) -> Option<Weapon> {
        self.weapons.insert(wpn.variant, wpn)
    }

    pub fn remove(&mut self, variant: WeaponVariant) -> Option<Weapon> {
        self.weapons.remove(&variant)
    }
}
