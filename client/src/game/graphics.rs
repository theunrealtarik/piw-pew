use graphics::{self, types::Color, Context, Viewport};
use lib::{assets::TEXTURES, types::Transform};
use opengl_graphics::{GlGraphics, GlyphCache};

use crate::configs::BACKGROUND_COLOR;

pub struct GameGraphics<'a> {
    gl: GlGraphics,
    glyphs: GlyphCache<'a>,
}

impl<'a> GameGraphics<'a> {
    pub fn new(gl: GlGraphics, glyphs: GlyphCache<'a>) -> Self {
        Self { gl, glyphs }
    }
    pub fn draw(&mut self, viewport: Viewport) -> Context {
        let context = self.gl.draw(viewport, |ctx, gl| {
            graphics::clear(BACKGROUND_COLOR, gl);

            return ctx;
        });

        context
    }

    pub fn rectangle<R>(&mut self, color: Color, rect: R, transform: Transform)
    where
        R: Into<graphics::types::Rectangle>,
    {
        graphics::rectangle(color, rect, transform, &mut self.gl);
    }
}
