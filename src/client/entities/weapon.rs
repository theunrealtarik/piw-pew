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
    pub accuracy: WeaponAccuracy,
    pub reload_time: Duration,
    pub fire_time: Duration,
    mag_size: u8,
    total_ammo: u8,
    pub curr_mag_size: u8,
    pub curr_total_ammo: u8,
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
        }
    }

    fn name(&self) -> &str {
        &self.name
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
        WeaponAccuracy::Moderate,
        Duration::from_millis(100),
        Duration::from_millis(1500),
        30,
        4
    );
    static ref WPN_STATS_SHOTPEW: WeaponStats = WeaponStats::new(
        "PUMP Shotpew",
        25,
        WeaponAccuracy::Low,
        Duration::from_millis(1000),
        Duration::from_millis(2000),
        5,
        5
    );
    static ref WPN_STATS_DEAN_1911: WeaponStats = WeaponStats::new(
        "DEAN 1911",
        25,
        WeaponAccuracy::High,
        Duration::from_millis(300),
        Duration::from_millis(1100),
        7,
        4
    );
    static ref WPN_STATS_PRRR: WeaponStats = WeaponStats::new(
        "PRRR",
        45,
        WeaponAccuracy::Low,
        Duration::from_millis(50),
        Duration::from_millis(2500),
        30,
        4
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
