use lib::types::Transform;
use piston::{RenderArgs, UpdateArgs};

use crate::game::GameGraphics;

pub trait GameObject {
    fn update(&mut self, args: &UpdateArgs);
    fn render(&mut self, args: &RenderArgs, transform: Transform, canvas: &mut GameGraphics);
}
