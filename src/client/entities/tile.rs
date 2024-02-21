use crate::game::TEXTURE;
use lib::types::Tile;
use nalgebra::{Point2, Vector2};
use raylib::math::Rectangle;

#[derive(Debug)]
pub struct GameWorldTile {
    pub variant: Tile,
    pub texture: TEXTURE,
    pub size: f32,
    pub src_rect: Rectangle,
    pub dest_rect: Rectangle,
    pub grid: Point2<u8>,
    pub position: Vector2<f32>,
}

impl GameWorldTile {
    pub fn new(
        variant: Tile,
        texture: TEXTURE,
        grid_x: u8,
        grid_y: u8,
        width: f32,
        height: f32,
        size: f32,
    ) -> Self {
        let wx = grid_x as f32 * size;
        let wy = grid_y as f32 * size;

        Self {
            variant,
            texture,
            size,
            src_rect: Rectangle::new(0.0, 0.0, width, height),
            dest_rect: Rectangle::new(wx, wy, size, size),
            grid: Point2::new(grid_x, grid_y),
            position: Vector2::new(wx, wy),
        }
    }

    pub fn get_dest_rect(&self, x: f32, y: f32) -> Rectangle {
        Rectangle::new(x, y, self.size, self.size)
    }
}
