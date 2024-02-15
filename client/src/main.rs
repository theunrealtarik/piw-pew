mod configs;
mod entities;
mod game;

use renet::{
    transport::{ClientAuthentication, NetcodeClientTransport},
    ConnectionConfig, DefaultChannel, RenetClient,
};
use slib::net::PROTOCOL_ID;
use std::{
    env::current_dir,
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};
use uuid::Uuid;

use env_logger;
use game::Game;
use lib::assets::Assets;

fn main() {
    env_logger::init();

    let _ = Assets::load(&current_dir().unwrap().join("assets"));
    let mut client = RenetClient::new(ConnectionConfig::default());

    let uuid = u64::from_le_bytes(Uuid::new_v4().as_bytes()[..8].try_into().unwrap());

    let server_addr: SocketAddr = "127.0.0.1:5000".parse().expect("failed to server socket");
    let socket = UdpSocket::bind("127.0.0.1:6969").unwrap();

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id: 0,
        user_data: None,
        protocol_id: PROTOCOL_ID,
    };

    let mut transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let mut window = raylib::init();

    window.title(configs::window::WINDOW_NAME);
    window.size(
        configs::window::WINDOW_WIDTH,
        configs::window::WINDOW_HEIGHT,
    );

    let (handle, thread) = window.build();
    let mut game = Game::new(handle, thread);

    game.handle.set_target_fps(60);
    while !game.handle.window_should_close() {
        game.update();
        game.render();

        let delta_time = Duration::from_millis(16);
        client.update(delta_time);
        transport.update(delta_time, &mut client).unwrap();

        if client.is_connected() {
            client.send_message(DefaultChannel::ReliableOrdered, "client text");
        }

        transport.send_packets(&mut client).unwrap();
        std::thread::sleep(delta_time);
    }
}
