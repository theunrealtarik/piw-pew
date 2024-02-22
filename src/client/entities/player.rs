use std::rc::Rc;

use lib::packets::GameNetworkPacket;
use lib::types::{Health, RVector2, SharedAssets};
use lib::{ENTITY_PLAYER_SIZE, ENTITY_WEAPON_SIZE, WORLD_TILE_SIZE};

use nalgebra::{Point2, Rotation2, Vector2};
use raylib::prelude::*;
use renet::DefaultChannel;

use crate::configs::{window, *};
use crate::core::*;
use crate::game::{Assets, GameNetwork};

use super::{Invenotry, Weapon};

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
    pub health: Health,
    pub ready: bool,
    assets: SharedAssets<Assets>,
}

impl Player {
    pub fn new(name: String, assets: SharedAssets<Assets>) -> Self {
        let (center_x, center_y) = Window::center();

        let rectangle = Rectangle::new(center_x, center_y, ENTITY_PLAYER_SIZE, ENTITY_PLAYER_SIZE);
        let origin = Vector2::new(rectangle.width / 2.0, rectangle.height / 2.0);

        Self {
            name,
            inventory: Invenotry::new(Rc::clone(&assets)),
            orientation: 0.0,
            rectangle,
            grid: Point2::new(0, 0),
            origin,
            camera: Camera2D {
                rotation: 0.0,
                zoom: 1.0,
                offset: RVector2::new(center_x, center_y),
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
            health: 100,
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

        let (offset_x, offset_y) = Window::center();
        self.camera.target.x = self.rectangle.x + player::PLAYER_CAMERA_OFFSET;
        self.camera.target.y = self.rectangle.y + player::PLAYER_CAMERA_OFFSET;
        // self.camera.offset.x = offset_x;
        // self.camera.offset.y = offset_y;

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

    pub fn on_shoot<F>(&mut self, handle: &RaylibHandle, f: F)
    where
        F: FnOnce(&Weapon, Vector2<f32>, f32),
    {
        let assets = self.assets.borrow();

        if handle.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            if let Some(wpn) = self.inventory.selected_weapon() {
                let buffer = assets.textures.get(&wpn.texture).unwrap();

                let origin = self.origin + Vector2::new(self.rectangle.x, self.rectangle.y);
                let theta = self.orientation.to_degrees();

                let flip_y = if theta.abs() <= 180.0 && theta.abs() > 90.0 {
                    true
                } else {
                    false
                };

                let (wpn_w, wpn_h) = (
                    buffer.width as f32 * ENTITY_WEAPON_SIZE,
                    buffer.height as f32 * ENTITY_WEAPON_SIZE,
                );

                let muzzle = Vector2::new(
                    origin.x + self.rectangle.width / 2.0 + wpn_w * wpn.muzzle.0,
                    origin.y + if flip_y { 1.0 } else { -1.0 } * (wpn_h / 2.0) * wpn.muzzle.1,
                );
                let coords = Rotation2::new(self.orientation) * (muzzle - origin) + origin;
                f(wpn, coords, self.orientation);
            }
        }
    }
}

impl NetUpdateHandle for Player {
    fn net_update(&mut self, handle: &RaylibHandle, network: &mut GameNetwork) {
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

        network.client.send_message(
            DefaultChannel::Unreliable,
            GameNetworkPacket::NET_PLAYER_ORIENTATION(
                network.transport.client_id(),
                self.orientation,
            )
            .serialized()
            .unwrap(),
        )
    }
}

impl RenderHandle for Player {
    fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        d.draw_rectangle_pro(self.rectangle, RVector2::zero(), 0.0, player::PLAYER_COLOR);

        let radius = self.rectangle.width / 2.0;
        let player_origin = Vector2::new(self.rectangle.x, self.rectangle.y).add_scalar(radius);

        self.inventory
            .render_weapon(d, player_origin, radius, self.orientation);

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
