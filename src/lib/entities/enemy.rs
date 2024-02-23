use raylib::prelude::*;

use nalgebra::Vector2;
use renet::ClientId;
use std::rc::Rc;

use super::Invenotry;

use crate::configs::*;
use crate::core::*;
use crate::types::*;
use crate::utils::*;

pub struct Enemy {
    pub id: ClientId,
    pub orientation: f32,
    pub rectangle: Rectangle,
    pub origin: Vector2<f32>,
    pub health: Health,
    pub inventory: Invenotry,
    assets: SharedAssets<GameAssets>,
}

impl Enemy {
    pub fn new(
        id: ClientId,
        x: f32,
        y: f32,
        orientation: f32,
        hp: Health,
        assets: SharedAssets<GameAssets>,
    ) -> Self {
        Self {
            id,
            orientation,
            rectangle: Rectangle::new(x, y, ENTITY_PLAYER_SIZE as f32, ENTITY_PLAYER_SIZE as f32),
            origin: Default::default(),
            health: hp,
            inventory: Invenotry::new(Rc::clone(&assets)),
            assets,
        }
    }
}

impl RenderHandle for Enemy {
    fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>)
    where
        Self: AssetsHandle,
    {
        d.draw_rectangle_pro(self.rectangle, RVector2::zero(), 0.0, Color::RED);

        let radius = self.rectangle.width / 2.0;
        let origin = Vector2::new(self.rectangle.x, self.rectangle.y).add_scalar(radius);
        self.inventory
            .render_weapon(d, &self.rectangle, self.orientation);
    }
}

impl AssetsHandle for Enemy {
    type GameAssets = SharedAssets<GameAssets>;

    fn get_assets(&self) -> Self::GameAssets {
        Rc::clone(&self.assets)
    }
}
