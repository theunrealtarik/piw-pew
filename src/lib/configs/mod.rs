use raylib::color::Color;

use crate::types::{Cash, Health};

pub static INITIAL_PAYLOAD_SIZE: usize = 255;
pub static INTIIAL_PLAYER_CASH: Cash = 200;

pub static PLAYER_KILL_REWARD: Cash = 400;
pub static PLAYER_DEATH_PENALTY: Cash = 700;
pub static PLAYER_HEATLH_COST: Cash = 450;
pub static PLAYER_AMMO_COST: Cash = 320;

pub static WORLD_TILE_SIZE: f32 = 70.0;
pub static ENTITY_PLAYER_SIZE: f32 = WORLD_TILE_SIZE * 0.8;
pub static ENTITY_WEAPON_SIZE: f32 = ENTITY_PLAYER_SIZE * 0.0018;
pub static ENTITY_PLAYER_MAX_HEALTH: Health = 100;
pub static ENTITY_PROJECTILE_SPEED: u32 = 50; // speed is the abs of velocity, it's not velocity (that's a death threat for every unity tutorial).
pub static ENTITY_PROJECTILE_RADIUS: f32 = 2.0;

pub static WINDOW_NAME: &str = "Piw Pew";
pub static WINDOW_HEIGHT: i32 = 650;
pub static WINDOW_WIDTH: i32 = 950;
pub static WINDOW_PADDING: i32 = 20;
pub static WINDOW_BACKGROUND_COLOR: Color = Color::new(17, 18, 19, 255);

pub static WINDOW_TOP_RIGHT_X: i32 = WINDOW_WIDTH - WINDOW_PADDING;
pub static WINDOW_TOP_RIGHT_Y: i32 = WINDOW_PADDING;
pub static WINDOW_TOP_LEFT_X: i32 = WINDOW_PADDING;
pub static WINDOW_TOP_LEFT_Y: i32 = WINDOW_PADDING;

pub static WINDOW_BOTTOM_RIGHT_X: i32 = WINDOW_WIDTH - WINDOW_PADDING;
pub static WINDOW_BOTTOM_RIGHT_Y: i32 = WINDOW_HEIGHT - WINDOW_PADDING;
pub static WINDOW_BOTTOM_LEFT_X: i32 = WINDOW_PADDING;
pub static WINDOW_BOTTOM_LEFT_Y: i32 = WINDOW_HEIGHT - WINDOW_PADDING;

pub static WINDOW_CENTER_X: f32 = WINDOW_WIDTH as f32 / 2.0;
pub static WINDOW_CENTER_Y: f32 = WINDOW_HEIGHT as f32 / 2.0;

pub static PLAYER_COLOR: Color = Color::new(246, 251, 255, 255);
pub static PLAYER_CAMERA_OFFSET: f32 = 20.0;
pub static PLAYER_INIT_VELOCITY_X: f32 = 450.0;
pub static PLAYER_INIT_VELOCITY_Y: f32 = 450.0;
