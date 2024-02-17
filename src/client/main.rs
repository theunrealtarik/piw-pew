mod configs;
mod entities;
mod game;

use configs::window;
use lib::{
    core::{RenderHandle, UpdateHandle},
    logging::Logger,
    net::{DELTA_TIME, PROTOCOL_ID},
};
use raylib::drawing::RaylibDraw;
use std::{net::SocketAddr, sync::Arc, time::SystemTime};

use env_logger;
use game::{Game, GameAssets, GameMenu, GameNetwork, GameSettings};

fn main() {
    env_logger::init_from_env(Logger::env());

    let current_dir = std::env::current_dir().unwrap();

    let server_addr: SocketAddr = "127.0.0.1:6969".parse().expect("failed to server socket");
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let mut window = raylib::init();

    window.title(configs::window::WINDOW_NAME);
    window.size(
        configs::window::WINDOW_WIDTH,
        configs::window::WINDOW_HEIGHT,
    );

    let (mut handle, thread) = window.build();
    let settings = GameSettings::load(&current_dir);

    let gs_loaded = match GameAssets::load(&mut handle, &thread, &current_dir.join("assets")) {
        Ok(assets) => assets,
        Err(_) => {
            log::error!("failed to load assets");
            std::process::exit(1);
        }
    };

    let assets = Arc::new(gs_loaded.assets);

    let mut network = match GameNetwork::connect(server_addr, current_time, PROTOCOL_ID) {
        Ok(net) => {
            log::info!("network layer is set");
            net
        }
        Err(_) => {
            log::error!("failed to setup network layer");
            std::process::exit(1);
        }
    };

    let mut menu = GameMenu::new(Arc::clone(&assets));
    let mut game = Game::new(Arc::clone(&assets), settings);

    while !handle.window_should_close() {
        let delta_time = DELTA_TIME;

        network.client.update(delta_time);
        network
            .transport
            .update(delta_time, &mut network.client)
            .unwrap();

        if network.client.is_connected() {
            game.update(&handle);
        }

        let mut d = handle.begin_drawing(&thread);
        d.clear_background(window::WINDOW_BACKGROUND_COLOR);

        if network.client.is_connecting() {
            menu.render(&mut d);
        } else if network.client.is_connected() {
            game.render(&mut d);
        }

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