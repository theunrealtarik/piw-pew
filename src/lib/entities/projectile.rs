use nalgebra::{Point2, Vector2};

use crate::prelude::*;
use crate::types::*;
use raylib::prelude::*;

#[derive(Debug, Clone)]
pub struct Projectile {
    pub id: RawProjectileId,
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub grid: Point2<i32>,
    pub orientation: f32,
}

impl Projectile {
    pub fn new(id: RawProjectileId, position: (f32, f32), speed: u32, orientation: f32) -> Self {
        let velocity = Vector2::new(
            speed as f32 * orientation.cos(),
            speed as f32 * orientation.sin(),
        );

        let grid = Point2::new(
            (position.0.round() / WORLD_TILE_SIZE) as i32,
            (position.1.round() / WORLD_TILE_SIZE) as i32,
        );

        Self {
            id,
            position: Vector2::new(position.0, position.1),
            velocity,
            grid,
            orientation,
        }
    }
}

impl RenderHandle for Projectile {
    fn render(&mut self, handle: &mut RaylibMode2D<RaylibDrawHandle>) {
        self.position += self.velocity;

        handle.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            ENTITY_PROJECTILE_RADIUS,
            Color::YELLOW,
        );
    }
}
