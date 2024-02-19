use crate::game::TEXTURE;
use lib::types::Tile;
use nalgebra::Vector2;
use raylib::math::Rectangle;

#[derive(Debug)]
pub struct GameWorldTile {
    pub variant: Tile,
    pub texture: TEXTURE,
    pub size: f32,
    pub src_rect: Rectangle,
    pub dest_rect: Rectangle,
    pub grid_position: Vector2<f32>,
    pub world_position: Vector2<f32>,
}

impl GameWorldTile {
    pub fn new(
        variant: Tile,
        texture: TEXTURE,
        grid_x: f32,
        grid_y: f32,
        width: f32,
        height: f32,
        size: f32,
    ) -> Self {
        let wx = grid_x * size;
        let wy = grid_y * size;

        Self {
            variant,
            texture,
            size,
            src_rect: Rectangle::new(0.0, 0.0, width, height),
            dest_rect: Rectangle::new(wx, wy, size, size),
            grid_position: Vector2::new(grid_x, grid_y),
            world_position: Vector2::new(wx, wy),
        }
    }

    pub fn get_dest_rect(&self, x: f32, y: f32) -> Rectangle {
        Rectangle::new(x, y, self.size, self.size)
    }
}
