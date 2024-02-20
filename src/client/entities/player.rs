use std::{cell::RefCell, rc::Rc};

use lib::types::{RVector2, SharedAssets};
use lib::PLAYER_TILE_SIZE;

use nalgebra::Vector2;
use raylib::prelude::*;

use crate::configs::{window, *};
use crate::core::*;
use crate::game::Assets;

use super::Invenotry;

#[allow(dead_code)]
pub struct Player {
    pub name: String,
    pub inventory: Invenotry,
    pub orientation: f32,
    pub rectangle: Rectangle,
    pub origin: Vector2<f32>,
    pub camera: Camera2D,
    pub velocity: Vector2<f32>,
    pub direction: Vector2<f32>,
    pub hp: i8,
    pub ready: bool,
    assets: SharedAssets<Assets>,
}

impl Player {
    pub fn new(name: String, assets: SharedAssets<Assets>) -> Self {
        let rectangle = Rectangle::new(
            window::WINDOW_CENTER_X,
            window::WINDOW_CENTER_Y,
            PLAYER_TILE_SIZE,
            PLAYER_TILE_SIZE,
        );
        let origin = Vector2::new(rectangle.width / 2.0, rectangle.height / 2.0);

        Self {
            name,
            inventory: Invenotry::new(),
            orientation: 0.0,
            rectangle,
            origin,
            camera: Camera2D {
                rotation: 0.0,
                zoom: 1.0,
                offset: RVector2::new(window::WINDOW_CENTER_X, window::WINDOW_CENTER_Y),
                target: RVector2::new(
                    rectangle.x + player::PLAYER_CAMERA_OFFSET,
                    rectangle.y + player::PLAYER_CAMERA_OFFSET,
                ),
            },
            ready: false,
            velocity: Vector2::new(
                player::PLAYER_INIT_VELOCITY_X,
                player::PLAYER_INIT_VELOCITY_Y,
            ),
            direction: Vector2::new(1.0, 1.0),
            hp: 100,
            assets,
        }
    }

    pub fn move_to(&mut self, position: Vector2<f32>) -> Vector2<f32> {
        self.rectangle.x = position.x;
        self.rectangle.y = position.y;

        self.camera.target.x = self.rectangle.x + player::PLAYER_CAMERA_OFFSET;
        self.camera.target.y = self.rectangle.y + player::PLAYER_CAMERA_OFFSET;

        Vector2::new(self.rectangle.x, self.rectangle.y)
    }

    pub fn on_move(&mut self, handle: &RaylibHandle) -> Vector2<f32> {
        let mut new_position = Vector2::new(self.rectangle.x, self.rectangle.y);
        let velocity = self.velocity.component_mul(&self.direction);
        let dt = handle.get_frame_time();
        new_position.x += velocity.x * dt;
        new_position.y += velocity.y * dt;

        new_position
    }
}

impl UpdateHandle for Player {
    fn update(&mut self, handle: &RaylibHandle) {
        self.direction = Vector2::new(0.0, 0.0);

        if handle.is_key_down(KeyboardKey::KEY_W) {
            self.direction.y = -1.0
        }

        if handle.is_key_down(KeyboardKey::KEY_S) {
            self.direction.y = 1.0
        }

        if handle.is_key_down(KeyboardKey::KEY_D) {
            self.direction.x = 1.0
        }

        if handle.is_key_down(KeyboardKey::KEY_A) {
            self.direction.x = -1.0
        }

        let player_pos = handle.get_world_to_screen2D(
            RVector2::new(self.rectangle.x, self.rectangle.y),
            self.camera,
        );

        let mouse_pos = handle.get_mouse_position();
        let mouse_x = mouse_pos.x as f32 - player_pos.x;
        let mouse_y = mouse_pos.y as f32 - player_pos.y;

        // self.orientation = mouse_y.atan2(mouse_x).to_degrees();
    }
}

impl RenderHandle for Player {
    fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_rectangle_pro(
            self.rectangle,
            RVector2::zero(),
            self.orientation,
            player::PLAYER_COLOR,
        );
    }
}

impl AssetsHandle for Player {
    type GameAssets = SharedAssets<Assets>;

    fn get_assets(&self) -> Self::GameAssets {
        Rc::clone(&self.assets)
    }
}
