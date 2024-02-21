use std::rc::Rc;

use lib::types::{RVector2, SharedAssets};
use lib::{ENTITY_PLAYER_SIZE, WORLD_TILE_SIZE};

use nalgebra::{Point2, Vector2};
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
    pub grid: Point2<i32>,
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
            ENTITY_PLAYER_SIZE,
            ENTITY_PLAYER_SIZE,
        );
        let origin = Vector2::new(rectangle.width / 2.0, rectangle.height / 2.0);

        Self {
            name,
            inventory: Invenotry::new(),
            orientation: 0.0,
            rectangle,
            grid: Point2::new(0, 0),
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
        self.grid = Point2::new(
            (position.x / WORLD_TILE_SIZE).round() as i32,
            (position.y / WORLD_TILE_SIZE).round() as i32,
        );

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
        let mouse_x = mouse_pos.x as f32 - (player_pos.x + ENTITY_PLAYER_SIZE / 2.0);
        let mouse_y = mouse_pos.y as f32 - (player_pos.y + ENTITY_PLAYER_SIZE / 2.0);

        self.orientation = mouse_y.atan2(mouse_x);
    }
}

impl RenderHandle for Player {
    fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_rectangle_pro(self.rectangle, RVector2::zero(), 0.0, player::PLAYER_COLOR);

        let radius = self.rectangle.width / 2.0;
        let player_origin = Vector2::new(self.rectangle.x, self.rectangle.y).add_scalar(radius);

        self.inventory.render_weapon(
            d,
            Rc::clone(&self.assets),
            player_origin,
            radius,
            self.orientation,
        );

        #[cfg(debug_assertions)]
        {
            let mouse_pos = d.get_screen_to_world2D(d.get_mouse_position(), self.camera);

            let lx = Vector2::new(
                self.rectangle.x,
                self.rectangle.y + ENTITY_PLAYER_SIZE / 2.0,
            );
            let ly = Vector2::new(
                self.rectangle.x + ENTITY_PLAYER_SIZE / 2.0,
                self.rectangle.y,
            );

            d.draw_line_v(
                RVector2::new(lx.x, lx.y),
                RVector2::new(lx.add_scalar(ENTITY_PLAYER_SIZE).x, lx.y),
                Color::BLUE,
            );

            d.draw_line_v(
                RVector2::new(ly.x, ly.y),
                RVector2::new(ly.x, ly.add_scalar(ENTITY_PLAYER_SIZE).y),
                Color::BLUE,
            );

            d.draw_line(
                player_origin.x as i32,
                player_origin.y as i32,
                mouse_pos.x as i32,
                mouse_pos.y as i32,
                Color::GREEN,
            );

            d.draw_circle_lines(
                player_origin.x as i32,
                player_origin.y as i32,
                radius,
                Color::RED,
            );

            d.draw_text(
                &format!("{:#?} {:#?}", self.grid.x, self.grid.y),
                player_origin.x as i32,
                player_origin.y as i32,
                12,
                Color::BLACK,
            );
        }
    }
}

impl AssetsHandle for Player {
    type GameAssets = SharedAssets<Assets>;

    fn get_assets(&self) -> Self::GameAssets {
        Rc::clone(&self.assets)
    }
}
