use raylib::prelude::*;

pub trait UpdateHandle {
    fn update(&mut self, handle: &RaylibHandle);
}

pub trait RenderHandle {
    fn render(&mut self, handle: &mut RaylibMode2D<RaylibDrawHandle>)
    where
        Self: AssetsHandle;
}

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

pub trait AssetsHandle {
    type GameAssets;
    fn get_assets(&self) -> Self::GameAssets;
}
