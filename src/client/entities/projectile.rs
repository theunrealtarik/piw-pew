use lib::WORLD_TILE_SIZE;
use nalgebra::{Point2, Vector2};

use crate::core::RenderHandle;
use raylib::prelude::*;

#[derive(Debug, Clone)]
pub struct Projectile {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub grid: Point2<i32>,
    pub orientation: f32,
    pub rectangle: Rectangle,
}

impl Projectile {
    pub fn new(position: Vector2<f32>, speed: u32, orientation: f32) -> Self {
        let velocity = Vector2::new(
            speed as f32 * orientation.cos(),
            speed as f32 * orientation.sin(),
        );

        let grid = Point2::new(
            (position.x.round() / WORLD_TILE_SIZE) as i32,
            (position.y.round() / WORLD_TILE_SIZE) as i32,
        );

        let rectangle = Rectangle::new(position.x, position.y, 5.0, 5.0);

        Self {
            position,
            velocity,
            grid,
            orientation,
            rectangle,
        }
    }
}

impl RenderHandle for Projectile {
    fn render(&mut self, handle: &mut RaylibMode2D<RaylibDrawHandle>) {
        self.position += self.velocity;

        self.rectangle.x = self.position.x;
        self.rectangle.y = self.position.y;

        handle.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            5.0,
            Color::WHITE,
        );
    }
}
