use std::rc::Rc;
use std::time::Instant;

use lib::packets::GameNetworkPacket;
use lib::types::{Health, RVector2, SharedAssets};
use lib::{ENTITY_PLAYER_SIZE, WORLD_TILE_SIZE};

use nalgebra::{Point2, Vector2};
use raylib::prelude::*;
use renet::DefaultChannel;

use crate::configs;
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
    pub reloading: bool,
    timers: Timer<Timers>,
    assets: SharedAssets<Assets>,
}

impl Player {
    pub fn new(name: String, assets: SharedAssets<Assets>) -> Self {
        let radius = ENTITY_PLAYER_SIZE / 2.0;
        let rectangle = Rectangle::new(
            configs::window::WINDOW_CENTER_X,
            configs::window::WINDOW_CENTER_Y,
            ENTITY_PLAYER_SIZE,
            ENTITY_PLAYER_SIZE,
        );
        let origin = Vector2::new(rectangle.width / 2.0, rectangle.height / 2.0);
        let offset = RVector2::new(
            configs::window::WINDOW_CENTER_X - radius,
            configs::window::WINDOW_CENTER_Y - radius,
        );

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
                offset,
                target: RVector2::new(rectangle.x, rectangle.y),
            },
            ready: false,
            reloading: false,
            velocity: Vector2::new(
                configs::player::PLAYER_INIT_VELOCITY_X,
                configs::player::PLAYER_INIT_VELOCITY_Y,
            ),
            direction: Vector2::new(1.0, 1.0),
            health: 100,
            timers: Timer::default(),
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

        self.camera.target.x = self.rectangle.x;
        self.camera.target.y = self.rectangle.y;

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
            if let Some(wpn) = self.inventory.selected_weapon_mut() {
                if !self.reloading
                    && self.timers.after(
                        Timers::WeaponShot(*wpn.stats.fire_time()),
                        *wpn.stats.fire_time(),
                    )
                {
                    let buffer = assets.textures.get(&wpn.texture).unwrap();
                    let muzzle = wpn.muzzle(buffer, &self.rectangle, self.orientation);

                    let amount = wpn
                        .curr_mag_ammo
                        .checked_sub(1)
                        .unwrap_or(wpn.curr_mag_ammo);
                    wpn.curr_mag_ammo = amount;

                    if wpn.curr_mag_ammo != 0 && wpn.curr_mag_ammo < wpn.stats.mag_size {
                        f(wpn, muzzle, self.orientation);
                    }
                }
            }
        }
    }
}

impl NetUpdateHandle for Player {
    fn net_update(&mut self, handle: &RaylibHandle, network: &mut GameNetwork) {
        if !self.ready {
            return;
        }

        if self.health <= 0 {
            network.client.send_message(
                DefaultChannel::ReliableUnordered,
                GameNetworkPacket::NET_PLAYER_DIED(network.transport.client_id())
                    .serialized()
                    .unwrap(),
            );

            return;
        }

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
        );

        if let Some(wpn) = self.inventory.selected_weapon_mut() {
            if handle.is_key_pressed(KeyboardKey::KEY_R) {
                self.reloading = true;
                self.timers.add(Timers::PlayerReloading, Instant::now());
            }

            if self.reloading
                && self
                    .timers
                    .after(Timers::PlayerReloading, *wpn.stats.reload_time())
            {
                self.reloading = false;
                wpn.reload();
            }
        }
    }
}

impl RenderHandle for Player {
    fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        if !self.ready || self.health <= 0 {
            return;
        }

        d.draw_rectangle_pro(
            self.rectangle,
            RVector2::zero(),
            0.0,
            configs::player::PLAYER_COLOR,
        );

        self.inventory
            .render_weapon(d, &self.rectangle, self.orientation);

        #[cfg(debug_assertions)]
        {
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

            let mouse_pos = d.get_screen_to_world2D(d.get_mouse_position(), self.camera);
            let origin = Vector2::new(self.rectangle.x, self.rectangle.y)
                .add_scalar(self.rectangle.width / 2.0);

            d.draw_line(
                origin.x as i32,
                origin.y as i32,
                mouse_pos.x as i32,
                mouse_pos.y as i32,
                Color::GREEN,
            );
            d.draw_text(
                &format!("{:#?} {:#?}", self.grid.x, self.grid.y),
                origin.x as i32,
                origin.y as i32,
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
