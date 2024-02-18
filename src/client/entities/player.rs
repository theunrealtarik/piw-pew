use std::{cell::RefCell, rc::Rc};

use crate::{
    configs::{entities, window},
    game::Assets,
};
use lib::core::*;
use nalgebra::{self, Point2, Scale2, Vector2};
use raylib::prelude::*;

#[allow(dead_code)]
pub struct Player {
    name: String,
    orientation: f32,
    position: Point2<f32>,
    scale: Scale2<f32>,
    velocity: Vector2<f32>,
    hp: i8,
    assets: Rc<RefCell<Assets>>,
}

impl Player {
    pub fn new(name: String, assets: Rc<RefCell<Assets>>) -> Self {
        Self {
            name,
            orientation: 0.0,
            position: Point2::new(0.0, 0.0),
            scale: Scale2::new(
                entities::WORLD_TILE_SIZE * 0.8,
                entities::WORLD_TILE_SIZE * 0.8,
            ),
            velocity: Vector2::new(10.0, 10.0),
            hp: 100,
            assets,
        }
    }
}

impl UpdateHandle for Player {
    fn update(&mut self, handle: &RaylibHandle) {
        if handle.is_key_down(KeyboardKey::KEY_W) {
            self.position.y -= self.velocity.y
        }

        if handle.is_key_down(KeyboardKey::KEY_S) {
            self.position.y += self.velocity.y
        }
        if handle.is_key_down(KeyboardKey::KEY_D) {
            self.position.x += self.velocity.x
        }

        if handle.is_key_down(KeyboardKey::KEY_A) {
            self.position.x -= self.velocity.x
        }

        self.position.x = nalgebra::clamp(
            self.position.x,
            self.scale.x / 2.0,
            window::WINDOW_WIDTH as f32 - self.scale.x / 2.0,
        );

        self.position.y = nalgebra::clamp(
            self.position.y,
            self.scale.y / 2.0,
            window::WINDOW_HEIGHT as f32 - self.scale.y / 2.0,
        );

        let mouse_pos = handle.get_mouse_position();
        let mouse_x = mouse_pos.x as f32 - self.position.x;
        let mouse_y = mouse_pos.y as f32 - self.position.y;

        self.orientation = mouse_y.atan2(mouse_x).to_degrees();
    }
}

impl RenderHandle for Player {
    fn render(&mut self, d: &mut RaylibDrawHandle) {
        let rect = Rectangle::new(self.position.x, self.position.y, self.scale.x, self.scale.y);
        let origin = ffi::Vector2 {
            x: rect.width / 2.0,
            y: rect.height / 2.0,
        };

        d.draw_rectangle_pro(rect, origin, self.orientation, entities::PLAYER_COLOR);
    }
}

impl AssetsHandle for Player {
    type GameAssets = Rc<RefCell<Assets>>;

    fn get_assets(&self) -> Self::GameAssets {
        Rc::clone(&self.assets)
    }
}

impl Entity for Player {
    fn get_position(&self) -> &Point2<f32> {
        &self.position
    }

    fn get_health(&self) -> &i8 {
        &self.hp
    }

    fn get_scale(&self) -> &Scale2<f32> {
        &self.scale
    }

    fn get_velocity(&self) -> &Vector2<f32> {
        &self.velocity
    }
}
