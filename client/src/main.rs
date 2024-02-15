extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

mod components;
mod configs;
mod entities;
mod game;
use std::env::current_dir;

use configs::*;
use game::*;
use lib::assets::{Assets, FONTS};

use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{ButtonEvent, EventLoop};
use piston_window::PistonWindow as Window;

use env_logger;

fn main() {
    env_logger::init();

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(WINDOW_NAME, [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .fullscreen(false)
        .resizable(false)
        .vsync(true)
        .automatic_close(true)
        .build()
        .unwrap();

    let game_assets = Assets::load(&current_dir().unwrap().join("assets"));
    let game_context = GameGraphics::new(
        GlGraphics::new(opengl),
        GlyphCache::new(FONTS::FNT_POPPINS.as_path(), (), TextureSettings::new()).unwrap(),
    );

    let mut game = Game::new(game_context);
    let mut events = Events::new(EventSettings::new()).ups(60);
    while let Some(e) = events.next(&mut window) {
        e.update(|args| game.update(args));
        e.render(|args| game.render(args));
        e.button(|args| game.button(&args));
    }
}
