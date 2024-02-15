#![allow(dead_code)]

use lib::types::Color;

pub const WINDOW_NAME: &str = "Piw Pew";
pub const WINDOW_HEIGHT: f64 = 600.0;
pub const WINDOW_WIDTH: f64 = 800.0;
pub const WINDOW_PADDING: f64 = 20.0;

pub const TEXT_FONT_NAME: &str = "Poppins-Regular.ttf";
pub const TEXT_FONT_SIZE: i32 = 16;

pub const PLAYER_SIZE: f64 = 40.0;

// color
pub const PLAYER_COLOR: Color = [0.69, 0.03, 1.0, 1.0];
pub const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
pub const BACKGROUND_COLOR: Color = [0.1, 0.1, 0.1, 1.0];
pub const ENEMY_COLOR: Color = [0.95, 0.52, 0.52, 1.0];
