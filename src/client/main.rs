mod configs;
mod core;
mod entities;
mod game;

use core::{NetRenderHandle, NetUpdateHandle, RenderHandle};
use lib::{
    logging::Logger,
    net::{DELTA_TIME, PROTOCOL_ID},
};

use env_logger;
use game::{Game, GameAssets, GameMenu, GameNetwork, GameSettings};
use raylib::prelude::*;

use std::{cell::RefCell, net::SocketAddr, rc::Rc, time::SystemTime};

static INITIAL_PAYLOAD_SIZE: usize = 255;

fn main() {
    env_logger::init_from_env(Logger::env());

    let current_dir = std::env::current_dir().unwrap();

    let server_addr: SocketAddr = "127.0.0.1:6969".parse().expect("failed to server socket");
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let (mut handle, thread) = raylib::init()
        .title(configs::window::WINDOW_NAME)
        .size(
            configs::window::WINDOW_WIDTH,
            configs::window::WINDOW_HEIGHT,
        )
        .build();

    let ga_loaded = match GameAssets::load(&mut handle, &thread, &current_dir.join("assets")) {
        Ok(assets) => assets,
        Err(_) => {
            log::error!("failed to load assets");
            std::process::exit(1);
        }
    };
    let assets = Rc::new(RefCell::new(ga_loaded.assets));

    let settings = GameSettings::load(&current_dir.join("settings.json"));
    let mut data: [u8; 256] = [0; 256];
    for (index, byte) in settings.username.bytes().enumerate() {
        if index >= INITIAL_PAYLOAD_SIZE {
            break;
        }

        data[index] = byte;
    }

    let mut network = match GameNetwork::connect(server_addr, current_time, PROTOCOL_ID, data) {
        Ok(net) => {
            log::info!("network layer is set");
            net
        }
        Err(_) => {
            log::error!("failed to setup network layer");
            std::process::exit(1);
        }
    };

    let mut menu = GameMenu::new(Rc::clone(&assets));
    let mut game = Game::new(assets.clone(), settings);

    while !handle.window_should_close() {
        let delta_time = DELTA_TIME;

        network.client.update(delta_time);
        network
            .transport
            .update(delta_time, &mut network.client)
            .unwrap();

        if network.client.is_connected() {
            game.net_update(&handle, &mut network);
        }

        let mut draw = handle.begin_drawing(&thread);
        draw.clear_background(configs::window::WINDOW_BACKGROUND_COLOR);

        let mut draw_2d = draw.begin_mode2D(game.player.camera);

        if network.client.is_connecting() {
            menu.render(&mut draw_2d);
        } else if network.client.is_connected() {
            game.render(&mut draw_2d);
        }

        std::mem::drop(draw_2d);

        match network.transport.send_packets(&mut network.client) {
            Ok(_) => {}
            Err(err) => {
                log::error!("failed to send packets");
                log::error!("{:#?}", err);
            }
        };
        std::thread::sleep(delta_time);
    }
}
