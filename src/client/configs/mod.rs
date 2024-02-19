#![allow(dead_code)]

pub static WORLD_TILE_SIZE: f32 = 50.0;

pub mod window {
    use lib::types::Color;

    pub const WINDOW_NAME: &str = "Piw Pew";
    pub const WINDOW_HEIGHT: i32 = 650;
    pub const WINDOW_WIDTH: i32 = 850;
    pub const WINDOW_PADDING: i32 = 20;
    pub const WINDOW_BACKGROUND_COLOR: Color = Color::new(17, 18, 19, 255);

    pub const WINDOW_TOP_RIGHT_X: i32 = WINDOW_WIDTH;
    pub const WINDOW_TOP_RIGHT_Y: i32 = 0;
    pub const WINDOW_TOP_LEFT_X: i32 = 0;
    pub const WINDOW_TOP_LEFT_Y: i32 = 0;

    pub const WINDOW_BOTTOM_RIGHT_X: i32 = WINDOW_WIDTH;
    pub const WINDOW_BOTTOM_RIGHT_Y: i32 = WINDOW_HEIGHT;
    pub const WINDOW_BOTTOM_LEFT_X: i32 = 0;
    pub const WINDOW_BOTTOM_LEFT_Y: i32 = WINDOW_HEIGHT;

    pub const WINDOW_CENTER_X: f32 = WINDOW_WIDTH as f32 / 2.0;
    pub const WINDOW_CENTER_Y: f32 = WINDOW_HEIGHT as f32 / 2.0;
}

pub mod player {
    use lib::types::Color;

    pub const PLAYER_COLOR: Color = Color::new(246, 251, 255, 255);
    pub const PLAYER_CAMERA_OFFSET: f32 = 20.0;
    pub const PLAYER_INIT_VELOCITY_X: f32 = 250.0;
    pub const PLAYER_INIT_VELOCITY_Y: f32 = 250.0;
}

pub mod font {
    use lib::types::Color;

    pub const STANDARD_TEXT_SIZE: i32 = 16;
    pub const STANDARD_TEXT_COLOR: Color = Color::new(255, 255, 255, 255);
    pub const STANDARD_TEXT_FONT_NAME: &str = "Poppins-Regular.ttf";
}
