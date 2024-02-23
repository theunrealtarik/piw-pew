#![allow(non_camel_case_types)]

use std::time::Duration;

use lazy_static::lazy_static;
use lib::types::WeaponVariant;

use nalgebra::{Rotation2, Vector2};
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
    mag_size: u8,
    total_ammo: u8,
    pub curr_mag_size: u8,
    pub curr_total_ammo: u8,
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
            curr_mag_size: mag_size,
            curr_total_ammo: mag_size * mags,
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
}

impl Weapon {
    pub fn new(variant: WeaponVariant) -> Self {
        let _ = WeaponStatsMapping::WPN_STATS_AKA_69.get();

        match variant {
            WeaponVariant::DEAN_1911 => Weapon {
                variant,
                texture: TEXTURE::WPN_DEAN,
                muzzle: (0.942, 0.685),
                stats: WeaponStatsMapping::WPN_STATS_DEAN_1911.get(),
            },
            WeaponVariant::AKA_69 => Weapon {
                variant,
                texture: TEXTURE::WPN_AKA,
                muzzle: (0.988, 0.173),
                stats: WeaponStatsMapping::WPN_STATS_AKA_69.get(),
            },
            WeaponVariant::SHOTPEW => Weapon {
                variant,
                texture: TEXTURE::WPN_SHOTPEW,
                muzzle: (0.988, 0.046),
                stats: WeaponStatsMapping::WPN_STATS_SHOTPEW.get(),
            },
            WeaponVariant::PRRR => Weapon {
                variant,
                texture: TEXTURE::WPN_PRRR,
                muzzle: (0.988, 0.372),
                stats: WeaponStatsMapping::WPN_STATS_PRRR.get(),
            },
        }
    }
}

// this bs is shared between the local player (Player) and other dudes (Enemy)
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

    pub fn add(&mut self, wpn: Weapon) -> Option<Weapon> {
        self.weapons.insert(wpn.variant, wpn)
    }

    pub fn remove(&mut self, variant: WeaponVariant) -> Option<Weapon> {
        self.weapons.remove(&variant)
    }

    pub fn set_mag_ammo<F>(f: F) -> ()
    where
        F: FnOnce(i32) -> i32,
    {
    }
}
