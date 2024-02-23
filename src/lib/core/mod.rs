use raylib::prelude::*;

mod loader;

pub use loader::*;

pub trait UpdateHandle {
    fn update(&mut self, handle: &RaylibHandle);
}

pub trait RenderHandle {
    fn render(&mut self, handle: &mut RaylibMode2D<RaylibDrawHandle>);
}

#[cfg_attr(feature = "client", features(cleint))]
pub trait NetUpdateHandle {
    type Network;
    fn net_update(&mut self, handle: &RaylibHandle, network: &mut Self::Network);
}

pub trait NetRenderHandle {
    type Network;
    fn net_render(
        &mut self,
        handle: &mut RaylibMode2D<RaylibDrawHandle>,
        network: &mut Self::Network,
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
