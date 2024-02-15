use lib::types::{Position, Transform};
use ncollide2d::shape::Capsule;
use piston::{Button, ButtonArgs, Key, RenderArgs, UpdateArgs};

use crate::{
    components::*,
    configs::{PLAYER_COLOR, PLAYER_SIZE},
    game::GameGraphics,
};

pub struct Player {
    pub orientation: f32,
    pub position: Position,
    pub health: Health,
    pub controller: Controller,
    pub shape: Capsule<f64>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            orientation: 0.0,
            position: Position::new(0.0, 0.0),
            health: Health::new(100.0),
            controller: Controller::new(),
            shape: Capsule::new(PLAYER_SIZE, 0.05),
        }
    }
}

impl GameObject for Player {
    fn update(&mut self, _args: &UpdateArgs) {}

    fn render(&mut self, _args: &RenderArgs, transform: Transform, canvas: &mut GameGraphics) {
        canvas.rectangle(
            PLAYER_COLOR,
            graphics::rectangle::square(self.position.x, self.position.y, PLAYER_SIZE),
            transform,
        );

        self.controller
            .on_hold(&Button::Keyboard(Key::W), || self.position.y -= 10.0);
        self.controller
            .on_hold(&Button::Keyboard(Key::D), || self.position.x += 10.0);
        self.controller
            .on_hold(&Button::Keyboard(Key::S), || self.position.y += 10.0);
        self.controller
            .on_hold(&Button::Keyboard(Key::A), || self.position.x -= 10.0);
    }
}

impl Entity for Player {
    fn position(&self) -> &Position {
        &self.position
    }

    fn health(&self) -> &Health {
        &self.health
    }
}

impl Controllable for Player {
    fn controller(&self) -> &Controller {
        &self.controller
    }

    fn controller_mut(&mut self) -> &mut Controller {
        &mut self.controller
    }

    fn button(&mut self, args: &ButtonArgs) -> () {
        let controller = self.controller_mut();
        controller.on_press(
            args,
            Some(|btn| match btn {
                Button::Keyboard(Key::E) => {
                    println!("pick up");
                }
                _ => {}
            }),
        );
        controller.on_release(args, None::<fn(_)>);
    }
}
