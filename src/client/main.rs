mod configs;
mod entities;
mod game;

use lib::{logging::Logger, net::PROTOCOL_ID, types::Color};
use raylib::drawing::RaylibDraw;
use renet::{
    transport::{ClientAuthentication, NetcodeClientTransport},
    ConnectionConfig, RenetClient,
};
use std::{
    env::current_dir,
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};
use uuid::Uuid;

use env_logger;
use game::{Game, GameAssets, GameNetwork};

fn main() {
    env_logger::init_from_env(Logger::env());

    let client = RenetClient::new(ConnectionConfig::default());
    let uuid = u64::from_le_bytes(Uuid::new_v4().as_bytes()[..8].try_into().unwrap());

    let server_addr: SocketAddr = "127.0.0.1:6969".parse().expect("failed to server socket");
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id: uuid,
        user_data: None,
        protocol_id: PROTOCOL_ID,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let mut window = raylib::init();

    window.title(configs::window::WINDOW_NAME);
    window.size(
        configs::window::WINDOW_WIDTH,
        configs::window::WINDOW_HEIGHT,
    );

    let (mut handle, thread) = window.build();

    let texture = match handle.load_texture(&thread, "assets/WPN_AKA") {
        Ok(texture) => texture,
        Err(_) => {
            log::error!("failed to load texture");
            std::process::exit(1);
        }
    };

    while !handle.window_should_close() {
        let mut d = handle.begin_drawing(&thread);

        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }

    // let r = match GameAssets::load(r, &current_dir().unwrap().join("assets")) {
    //     Ok(assets) => assets,
    //     Err(_) => {
    //         log::error!("failed to load assets");
    //         std::process::exit(1);
    //     }
    // };
    //
    // let net = GameNetwork::new(transport, client);
    // let mut game = Game::new(r.handle, r.thread, r.assets, net);
    //
    // while !game.handle.window_should_close() {
    //     game.run();
    // }
}
