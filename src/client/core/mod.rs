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
