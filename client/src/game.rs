use crate::configs::window;
use crate::entities::Player;

use lib::core::{Render, Update};
use raylib::{drawing::RaylibDraw, rgui::RaylibDrawGui, RaylibHandle, RaylibThread};

pub struct GameState {
    local_player: Player,
}

pub struct Game {
    state: GameState,
    pub handle: RaylibHandle,
    pub thread: RaylibThread,
}

impl Game {
    pub fn new(handle: RaylibHandle, thread: RaylibThread) -> Self {
        Self {
            state: GameState {
                local_player: Player::new(),
            },
            handle,
            thread,
        }
    }

    pub fn update(&mut self) {
        self.state.local_player.update(&mut self.handle);
    }

    pub fn render(&mut self) {
        let mut d = self.handle.begin_drawing(&self.thread);

        d.gui_enable();
        d.clear_background(window::WINDOW_BACKGROUND_COLOR);
        d.draw_fps(
            window::WINDOW_TOP_LEFT_X + window::WINDOW_PADDING,
            window::WINDOW_TOP_LEFT_Y + window::WINDOW_PADDING,
        );

        self.state.local_player.render(&mut d);
    }
}
