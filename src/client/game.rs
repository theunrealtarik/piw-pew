use crate::configs::window;
use crate::entities::Player;

use lib::{
    core::{Render, Update},
    types::Color,
};
use raylib::{drawing::RaylibDraw, rgui::RaylibDrawGui, RaylibHandle, RaylibThread};
use renet::{transport::NetcodeClientTransport, DefaultChannel, RenetClient};

use lib::net::DELTA_TIME;

pub struct GameState {
    player: Player,
}

pub struct Game {
    pub local: GameState,
    pub handle: RaylibHandle,
    pub thread: RaylibThread,
    pub network: GameNetwork,
}

impl Game {
    pub fn new(handle: RaylibHandle, thread: RaylibThread, network: GameNetwork) -> Self {
        Self {
            local: GameState {
                player: Player::new(),
            },
            handle,
            thread,
            network,
        }
    }

    pub fn update(&mut self) {
        if self.network.client.is_connecting() {
            return;
        }

        self.local.player.update(&mut self.handle);
    }

    pub fn render(&mut self) {
        let mut d = self.handle.begin_drawing(&self.thread);

        d.gui_enable();
        d.clear_background(window::WINDOW_BACKGROUND_COLOR);

        if self.network.client.is_connecting() {
            self.network.tries += 1;

            let mut fallback = String::from("Connecting ...");
            if self.network.tries > 0 {
                fallback = format!("Connecting .... (tries: {})", self.network.tries);
            }

            d.draw_text(
                &fallback,
                window::WINDOW_PADDING,
                window::WINDOW_PADDING,
                26,
                Color::WHITE,
            );
        }

        if self.network.client.is_connected() {
            self.local.player.render(&mut d);
        }
    }

    pub fn run(&mut self) {
        let delta_time = DELTA_TIME;
        self.network.client.update(delta_time);
        self.network
            .transport
            .update(delta_time, &mut self.network.client)
            .unwrap();

        self.update();
        self.render();

        if self.network.client.is_connected() {
            self.network
                .client
                .send_message(DefaultChannel::ReliableOrdered, "client text");
        }

        match self
            .network
            .transport
            .send_packets(&mut self.network.client)
        {
            Ok(_) => {}
            Err(err) => {
                log::error!("failed to send packets");
                log::error!("{:#?}", err);
            }
        };
        std::thread::sleep(delta_time);
    }
}

pub struct GameNetwork {
    transport: NetcodeClientTransport,
    client: RenetClient,
    tries: u8,
}

impl GameNetwork {
    pub fn new(transport: NetcodeClientTransport, client: RenetClient) -> Self {
        Self {
            transport,
            client,
            tries: 0,
        }
    }
}
