#![allow(dead_code)]

use crate::game::GameNetwork;
use raylib::prelude::*;

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

pub struct Window;
impl Window {
    pub fn current_display() -> i32 {
        window::get_current_monitor()
    }

    pub fn width() -> i32 {
        window::get_monitor_width(Self::current_display())
    }

    pub fn height() -> i32 {
        window::get_monitor_height(Self::current_display())
    }

    pub fn center() -> (f32, f32) {
        let w = Self::width() as f32 / 2.0;
        let h = Self::height() as f32 / 2.0;

        (w / 2.0, h / 2.0)
    }
}
