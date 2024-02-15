use ::graphics::Transformed;
use piston::{ButtonArgs, RenderArgs, UpdateArgs};

use crate::components::{Controllable, GameObject};
use crate::configs::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::entities::Player;

mod graphics;
pub use graphics::GameGraphics;

pub struct Game<'a> {
    pub graphics: GameGraphics<'a>,
    pub local_player: Player,
}

impl<'a> Game<'a> {
    pub fn new(graphics: GameGraphics<'a>) -> Self {
        Self {
            graphics,
            local_player: Player::new(),
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.local_player.update(args);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let ctx = self.graphics.draw(args.viewport());
        let transform = ctx.transform.trans(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);

        self.local_player
            .render(args, transform, &mut self.graphics);
    }

    pub fn button(&mut self, args: &ButtonArgs) {
        self.local_player.button(args);
    }
}
