use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
};
use slib::net::{DELTA_TIME, PROTOCOL_ID};
use std::{
    net::{IpAddr, SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    time::SystemTime,
};

pub struct Server;

use lib::{Connection, ServerState};

impl Server {
    pub fn run(connection: &Arc<Connection>, state: &Arc<Mutex<ServerState>>) {
        let public_addr = SocketAddr::new(IpAddr::V4(connection.addr), connection.port);

        let connection_config = ConnectionConfig::default();
        let mut server: RenetServer = RenetServer::new(connection_config);

        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let server_config = ServerConfig {
            current_time,
            max_clients: 64,
            protocol_id: PROTOCOL_ID,
            public_addresses: vec![public_addr],
            authentication: ServerAuthentication::Unsecure,
        };
        let socket: UdpSocket = UdpSocket::bind(public_addr).unwrap();

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
            let delta_time = DELTA_TIME;
            server.update(delta_time);
            transport.update(delta_time, &mut server).unwrap();

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
}
