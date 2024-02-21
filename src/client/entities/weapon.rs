#![allow(non_camel_case_types)]

use std::time::Duration;

use lazy_static::lazy_static;
use lib::packets::WeaponVariant;

use crate::game::TEXTURE;

#[derive(Debug, Clone)]
pub enum WeaponAccuracy {
    Low,
    Moderate,
    High,
}

#[derive(Debug)]
pub struct WeaponStats {
    name: &'static str,
    damage: u8,
    pub fire_rate: u8,
    pub accuracy: WeaponAccuracy,
    pub reload_time: Duration,
    mag_size: u8,
    total_ammo: u8,
    pub curr_mag_size: u8,
    pub curr_total_ammo: u8,
}

impl WeaponStats {
    pub fn new(
        name: &'static str,
        damage: u8,
        fire_rate: u8,
        accuracy: WeaponAccuracy,
        reload_time: Duration,
        mag_size: u8,
        mags: u8,
    ) -> Self {
        Self {
            name,
            damage,
            fire_rate,
            accuracy,
            reload_time,
            mag_size,
            total_ammo: mag_size * mags,
            curr_mag_size: mag_size,
            curr_total_ammo: mag_size * mags,
        }
    }

    fn damage(&self) -> &u8 {
        &self.damage
    }
}

// weapons stats
lazy_static! {
    static ref WPN_STATS_AKA_69: WeaponStats = WeaponStats::new(
        "AKA-69",
        40,
        5,
        WeaponAccuracy::Moderate,
        Duration::from_millis(1500),
        30,
        4
    );
    static ref WPN_STATS_SHOTPEW: WeaponStats = WeaponStats::new(
        "PUMP Shotpew",
        25,
        1,
        WeaponAccuracy::Low,
        Duration::from_millis(2000),
        5,
        5
    );
    static ref WPN_STATS_DEAN_1911: WeaponStats = WeaponStats::new(
        "DEAN 1911",
        25,
        7,
        WeaponAccuracy::High,
        Duration::from_millis(1100),
        7,
        4
    );
    static ref WPN_STATS_PRRR: WeaponStats = WeaponStats::new(
        "PRRR",
        45,
        15,
        WeaponAccuracy::Low,
        Duration::from_millis(2500),
        30,
        4
    );
}

#[derive(Debug)]
pub struct Weapon {
    pub variant: WeaponVariant,
    pub texture: TEXTURE,
    pub stats: WeaponStats,
}

impl Weapon {
    pub fn new(variant: WeaponVariant) -> Self {
        match variant {
            WeaponVariant::DEAN_1911 => Weapon {
                variant,
                texture: TEXTURE::WPN_DEAN,
                stats: WeaponStats {
                    name: WPN_STATS_DEAN_1911.name,
                    damage: WPN_STATS_DEAN_1911.damage,
                    fire_rate: WPN_STATS_DEAN_1911.fire_rate,
                    accuracy: WPN_STATS_DEAN_1911.accuracy.clone(),
                    reload_time: WPN_STATS_DEAN_1911.reload_time,
                    mag_size: WPN_STATS_DEAN_1911.mag_size,
                    total_ammo: WPN_STATS_DEAN_1911.total_ammo,
                    curr_mag_size: WPN_STATS_DEAN_1911.curr_mag_size,
                    curr_total_ammo: WPN_STATS_DEAN_1911.curr_total_ammo,
                },
            },
            WeaponVariant::AKA_69 => Weapon {
                variant,
                texture: TEXTURE::WPN_AKA,
                stats: WeaponStats {
                    name: WPN_STATS_AKA_69.name,
                    damage: WPN_STATS_AKA_69.damage,
                    fire_rate: WPN_STATS_AKA_69.fire_rate,
                    accuracy: WPN_STATS_AKA_69.accuracy.clone(),
                    reload_time: WPN_STATS_AKA_69.reload_time,
                    mag_size: WPN_STATS_AKA_69.mag_size,
                    total_ammo: WPN_STATS_AKA_69.total_ammo,
                    curr_mag_size: WPN_STATS_AKA_69.curr_mag_size,
                    curr_total_ammo: WPN_STATS_AKA_69.curr_total_ammo,
                },
            },
            WeaponVariant::SHOTPEW => Weapon {
                variant,
                texture: TEXTURE::WPN_SHOTPEW,
                stats: WeaponStats {
                    name: WPN_STATS_SHOTPEW.name,
                    damage: WPN_STATS_SHOTPEW.damage,
                    fire_rate: WPN_STATS_SHOTPEW.fire_rate,
                    accuracy: WPN_STATS_SHOTPEW.accuracy.clone(),
                    reload_time: WPN_STATS_SHOTPEW.reload_time,
                    mag_size: WPN_STATS_SHOTPEW.mag_size,
                    total_ammo: WPN_STATS_SHOTPEW.total_ammo,
                    curr_mag_size: WPN_STATS_SHOTPEW.curr_mag_size,
                    curr_total_ammo: WPN_STATS_SHOTPEW.curr_total_ammo,
                },
            },
            WeaponVariant::PRRR => Weapon {
                variant,
                texture: TEXTURE::WPN_PRRR,
                stats: WeaponStats {
                    name: WPN_STATS_PRRR.name,
                    damage: WPN_STATS_PRRR.damage,
                    fire_rate: WPN_STATS_PRRR.fire_rate,
                    accuracy: WPN_STATS_PRRR.accuracy.clone(),
                    reload_time: WPN_STATS_PRRR.reload_time,
                    mag_size: WPN_STATS_PRRR.mag_size,
                    total_ammo: WPN_STATS_PRRR.total_ammo,
                    curr_mag_size: WPN_STATS_PRRR.curr_mag_size,
                    curr_total_ammo: WPN_STATS_PRRR.curr_total_ammo,
                },
            },
        }
    }
}
