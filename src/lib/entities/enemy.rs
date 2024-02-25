use raylib::prelude::*;

use nalgebra::Vector2;
use renet::ClientId;
use std::rc::Rc;

use super::WeaponVariant;

use crate::configs::*;
use crate::core::*;
use crate::types::*;
use crate::utils::*;

pub struct Enemy {
    pub id: ClientId,
    pub orientation: Orientation,
    pub rectangle: Rectangle,
    pub origin: Vector2<f32>,
    pub health: Health,
    pub weapon: Option<WeaponVariant>,
    assets: SharedAssets<GameAssets>,
}

impl Enemy {
    pub fn new(
        id: ClientId,
        x: f32,
        y: f32,
        orientation: Orientation,
        hp: Health,
        assets: SharedAssets<GameAssets>,
    ) -> Self {
        Self {
            id,
            orientation,
            rectangle: Rectangle::new(x, y, ENTITY_PLAYER_SIZE as f32, ENTITY_PLAYER_SIZE as f32),
            origin: Default::default(),
            health: hp,
            weapon: None,
            assets,
        }
    }
}

impl RenderHandle for Enemy {
    fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>)
    where
        Self: AssetsHandle,
    {
        let assets = Rc::clone(&self.assets);
        d.draw_rectangle_pro(self.rectangle, RVector2::zero(), 0.0, Color::WHITE);
        if let Some(wpn) = self.weapon {
            wpn.weapon_instance()
                .render_weapon(d, &self.rectangle, self.orientation, assets);
        }
    }
}

impl AssetsHandle for Enemy {
    type GameAssets = SharedAssets<GameAssets>;

    fn get_assets(&self) -> Self::GameAssets {
        Rc::clone(&self.assets)
    }
}
