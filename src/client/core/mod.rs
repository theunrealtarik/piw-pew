#![allow(dead_code)]

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::game::GameNetwork;
use raylib::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Timers {
    WeaponShot(Duration),
    PlayerReloading,
    PlayerRepsawn,
}

pub trait UpdateHandle {
    fn update(&mut self, handle: &RaylibHandle);
}

pub trait RenderHandle {
    fn render(&mut self, handle: &mut RaylibMode2D<RaylibDrawHandle>);
}

pub trait NetUpdateHandle {
    fn net_update(&mut self, handle: &RaylibHandle, network: &mut GameNetwork);
}

pub trait NetRenderHandle {
    fn net_render(
        &mut self,
        handle: &mut RaylibMode2D<RaylibDrawHandle>,
        network: &mut GameNetwork,
    ) where
        Self: AssetsHandle;
}

pub trait UserInterfaceHandle {
    fn display(&mut self, handle: &mut RaylibDrawHandle);
}

pub trait AssetsHandle {
    type GameAssets;
    fn get_assets(&self) -> Self::GameAssets;
}

pub struct Timer<T: Copy + PartialEq + Eq + std::hash::Hash> {
    value: HashMap<T, Instant>,
}

impl<T: Copy + PartialEq + Eq + std::hash::Hash> Default for Timer<T> {
    fn default() -> Self {
        Self {
            value: HashMap::new(),
        }
    }
}

impl<T: Copy + Eq + std::hash::Hash> Timer<T> {
    pub fn after(&mut self, id: T, duration: Duration) -> bool {
        match self.value.get_mut(&id) {
            Some(instant) => {
                let now = Instant::now();
                let dt = now - *instant;

                if dt >= duration {
                    *instant = Instant::now();
                    true
                } else {
                    false
                }
            }
            None => {
                self.value.insert(id, Instant::now());
                true
            }
        }
    }
}
