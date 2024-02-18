use std::{cell::RefCell, rc::Rc};

use lib::types::RVector2;
use nalgebra::Vector2;
use raylib::prelude::*;

use crate::configs::{window, *};
use crate::core::*;
use crate::game::Assets;

#[allow(dead_code)]
pub struct Player {
    pub name: String,
    pub orientation: f32,
    pub rectangle: Rectangle,
    pub camera: Camera2D,
    pub velocity: Vector2<f32>,
    pub hp: i8,
    pub visible: bool,
    assets: Rc<RefCell<Assets>>,
}

impl Player {
    pub fn new(name: String, assets: Rc<RefCell<Assets>>) -> Self {
        let rectangle = Rectangle::new(0.0, 0.0, WORLD_TILE_SIZE * 0.8, WORLD_TILE_SIZE * 0.8);
        Self {
            name,
            orientation: 0.0,
            rectangle,
            camera: Camera2D {
                rotation: 0.0,
                zoom: 1.0,
                offset: RVector2::new(window::WINDOW_CENTER_X, window::WINDOW_CENTER_Y),
                target: RVector2::new(
                    rectangle.x + player::PLAYER_CAMERA_OFFSET,
                    rectangle.y + player::PLAYER_CAMERA_OFFSET,
                ),
            },
            visible: false,
            velocity: Vector2::new(10.0, 10.0),
            hp: 100,
            assets,
        }
    }
}

impl UpdateHandle for Player {
    fn update(&mut self, handle: &RaylibHandle) {
        let mut direction = Vector2::new(0.0, 0.0);

        if handle.is_key_down(KeyboardKey::KEY_W) {
            direction.y = -1.0
        }
        if handle.is_key_down(KeyboardKey::KEY_S) {
            direction.y = 1.0
        }
        if handle.is_key_down(KeyboardKey::KEY_D) {
            direction.x = 1.0
        }
        if handle.is_key_down(KeyboardKey::KEY_A) {
            direction.x = -1.0
        }

        let velocity = self.velocity.component_mul(&direction);

        self.rectangle.x += velocity.x;
        self.rectangle.y += velocity.y;

        let mouse_pos = handle.get_mouse_position();
        let mouse_x = mouse_pos.x as f32 - self.rectangle.x;
        let mouse_y = mouse_pos.y as f32 - self.rectangle.y;

        self.orientation = mouse_y.atan2(mouse_x).to_degrees();

        self.camera.target.x = self.rectangle.x + player::PLAYER_CAMERA_OFFSET;
        self.camera.target.y = self.rectangle.y + player::PLAYER_CAMERA_OFFSET;
    }
}

impl RenderHandle for Player {
    fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        let origin = math::Vector2 {
            x: self.rectangle.width / 2.0,
            y: self.rectangle.height / 2.0,
        };

        d.draw_rectangle_pro(
            self.rectangle,
            origin,
            self.orientation,
            player::PLAYER_COLOR,
        );
    }
}

impl AssetsHandle for Player {
    type GameAssets = Rc<RefCell<Assets>>;

    fn get_assets(&self) -> Self::GameAssets {
        Rc::clone(&self.assets)
    }
}
