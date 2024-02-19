#![allow(non_camel_case_types, dead_code)]
use std::time::Duration;

use once_cell::sync::Lazy;

use crate::core::RenderHandle;

pub enum WeaponAccuracy {
    Low,
    Moderate,
    High,
}

pub struct WeaponStats {
    name: &'static str,
    damage: u8,
    fire_rate: u8,
    accuracy: WeaponAccuracy,
    reload_time: Duration,
    max_total_ammo: u8,
    max_mag_ammo: u8,
    mag_size: u8,
}

impl WeaponStats {
    pub fn new(
        name: &'static str,
        damage: u8,
        fire_rate: u8,
        accuracy: WeaponAccuracy,
        reload_time: Duration,
        mag_size: u8,
    ) -> Self {
        Self {
            name,
            damage,
            fire_rate,
            accuracy,
            reload_time,
            max_total_ammo: mag_size * 3,
            max_mag_ammo: mag_size,
            mag_size: 0,
        }
    }
}

pub enum Weapon {
    DEAN_1911(WeaponStats),
    AKA_69(WeaponStats),
    SHOTPEW(WeaponStats),
    PRRR(WeaponStats),
}

impl RenderHandle for Weapon {
    fn render(
        &mut self,
        handle: &mut raylib::prelude::RaylibMode2D<raylib::prelude::RaylibDrawHandle>,
    ) {
    }
}

pub static WEAPON_DEAD_1911: Lazy<Weapon> = Lazy::new(|| {
    Weapon::DEAN_1911(WeaponStats::new(
        "Dean 1911",
        22,
        3,
        WeaponAccuracy::Moderate,
        Duration::from_millis(1200),
        7,
    ))
});

pub static WEAPON_AKA_69: Lazy<Weapon> = Lazy::new(|| {
    Weapon::DEAN_1911(WeaponStats::new(
        "AKA-69",
        42,
        1,
        WeaponAccuracy::Low,
        Duration::from_millis(2500),
        30,
    ))
});

pub static WEAPON_SHOTPEW: Lazy<Weapon> = Lazy::new(|| {
    Weapon::SHOTPEW(WeaponStats::new(
        "PUMP Shotpew",
        64,
        1,
        WeaponAccuracy::High,
        Duration::from_millis(2900),
        5,
    ))
});

pub static WEAPON_PRRR: Lazy<Weapon> = Lazy::new(|| {
    Weapon::DEAN_1911(WeaponStats::new(
        "American PRRR",
        120,
        10,
        WeaponAccuracy::Low,
        Duration::from_millis(3700),
        120,
    ))
});
