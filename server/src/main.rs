use env_logger;
use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
};
use slib::net::PROTOCOL_ID;
use std::{
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};

fn main() {
    env_logger::init();

    let mut server = RenetServer::new(ConnectionConfig::default());

    let server_addr: SocketAddr = format!("0.0.0.0:{}", 5000).parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();

    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let mut transport = match NetcodeServerTransport::new(server_config, socket) {
        Ok(t) => {
            log::info!("transporting layer is setup");
            t
        }
        Err(_) => {
            log::error!("failed to setup transporting layer");
            std::process::exit(1);
        }
    };
    loop {
        let delta_time = Duration::from_millis(16);
        server.update(delta_time);

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    log::info!("client connected {}", client_id);
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    log::info!("client disconnected {} {}", client_id, reason);
                }
            }
        }

        server.broadcast_message(DefaultChannel::ReliableOrdered, "server message");
        transport.send_packets(&mut server);
        std::thread::sleep(delta_time);
    }
}
