use crate::configs::{entities, font, window};
use lib::core::*;
use nalgebra::{self, Point2, Scale2, Vector2};
use raylib::prelude::*;

pub struct Player {
    orientation: f32,
    position: Point2<f32>,
    scale: Scale2<f32>,
    velocity: Vector2<f32>,
    controller: Controller,
    health: Health,
    color: Color,
}

impl Player {
    pub fn new() -> Self {
        Self {
            orientation: 0.0,
            position: Point2::new(0.0, 0.0),
            health: Health::new(100.0),
            controller: Controller::new(),
            scale: Scale2::new(40.0, 40.0),
            velocity: Vector2::new(10.0, 10.0),
            color: entities::PLAYER_COLOR,
        }
    }
}

impl Update for Player {
    fn update(&mut self, handle: &mut RaylibHandle) {
        self.controller.on_hold(handle, KeyboardKey::KEY_W, || {
            self.position.y -= self.velocity.y
        });
        self.controller.on_hold(handle, KeyboardKey::KEY_S, || {
            self.position.y += self.velocity.y
        });
        self.controller.on_hold(handle, KeyboardKey::KEY_D, || {
            self.position.x += self.velocity.x
        });
        self.controller.on_hold(handle, KeyboardKey::KEY_A, || {
            self.position.x -= self.velocity.x
        });

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

impl Render for Player {
    fn render(&mut self, d: &mut RaylibDrawHandle) {
        let rect = Rectangle::new(self.position.x, self.position.y, self.scale.x, self.scale.y);
        let origin = ffi::Vector2 {
            x: rect.width / 2.0,
            y: rect.height / 2.0,
        };

        d.draw_rectangle_pro(rect, origin, self.orientation, entities::PLAYER_COLOR);
        d.draw_text(
            &format!("x: {:?} y: {:?}", self.position.x, self.position.y),
            window::WINDOW_TOP_LEFT_X + window::WINDOW_PADDING,
            window::WINDOW_TOP_LEFT_Y + window::WINDOW_PADDING * 2,
            font::STANDARD_TEXT_SIZE,
            font::STANDARD_TEXT_COLOR,
        )
    }
}

impl Entity for Player {
    fn get_position(&self) -> &Point2<f32> {
        &self.position
    }

    fn get_health(&self) -> &Health {
        &self.health
    }

    fn get_scale(&self) -> &Scale2<f32> {
        &self.scale
    }

    fn get_velocity(&self) -> &Vector2<f32> {
        &self.velocity
    }
}
