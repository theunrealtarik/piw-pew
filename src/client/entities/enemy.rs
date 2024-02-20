use std::{cell::RefCell, rc::Rc};

use lib::{types::RVector2, PLAYER_TILE_SIZE};
use nalgebra::Vector2;

use raylib::prelude::*;
use renet::ClientId;

use crate::{
    core::{AssetsHandle, RenderHandle},
    game::Assets,
};

use super::Invenotry;

pub struct Enemy {
    pub id: ClientId,
    pub orientation: f32,
    pub rectangle: Rectangle,
    pub origin: Vector2<f32>,
    pub hp: u8,
    pub inventory: Invenotry,
    assets: Rc<RefCell<Assets>>,
}

impl Enemy {
    pub fn new(
        id: ClientId,
        x: f32,
        y: f32,
        orientation: f32,
        hp: u8,
        assets: Rc<RefCell<Assets>>,
    ) -> Self {
        Self {
            id,
            orientation,
            rectangle: Rectangle::new(x, y, PLAYER_TILE_SIZE as f32, PLAYER_TILE_SIZE as f32),
            origin: Default::default(),
            hp,
            inventory: Invenotry::new(),
            assets,
        }
    }
}

impl RenderHandle for Enemy {
    fn render(&mut self, d: &mut RaylibMode2D<RaylibDrawHandle>)
    where
        Self: AssetsHandle,
    {
        d.draw_rectangle_pro(
            self.rectangle,
            RVector2::zero(),
            self.orientation,
            Color::RED,
        );
    }
}

impl AssetsHandle for Enemy {
    type GameAssets = Rc<RefCell<Assets>>;

    fn get_assets(&self) -> Self::GameAssets {
        Rc::clone(&self.assets)
    }
}
