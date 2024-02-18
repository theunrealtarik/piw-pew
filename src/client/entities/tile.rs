use crate::game::TEXTURE;
use raylib::math::Rectangle;

#[derive(Debug)]
pub struct GameWorldTile {
    pub texture: TEXTURE,
    pub scale: f32,
    pub rectangle: Rectangle,
}

impl GameWorldTile {
    pub fn new(texture: TEXTURE, width: f32, height: f32, scale: f32) -> Self {
        Self {
            texture,
            scale,
            rectangle: Rectangle::new(0.0, 0.0, width, height),
        }
    }

    pub fn rec_scale(&self, x: f32, y: f32) -> Rectangle {
        Rectangle::new(
            x,
            y,
            self.rectangle.width * self.scale,
            self.rectangle.height * self.scale,
        )
    }
}
