#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod server;
use log;
use slib::logging::Logger;

use lib::{Connection, ServerState};
use server::Server;
use std::{
    net::Ipv4Addr,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    env_logger::init_from_env(Logger::env());

    let connection = Arc::new(Connection {
        addr: Ipv4Addr::new(0, 0, 0, 0),
        port: 6969,
    });

    let state = Arc::new(Mutex::new(ServerState {}));
    let app = tauri::Builder::default();

    let s_conn = Arc::clone(&connection);

    thread::spawn(move || {
        log::info!("server thread");
        Server::run(&s_conn, &state);
    });

    match app.run(tauri::generate_context!()) {
        Ok(_) => log::info!("app is running"),
        Err(_) => log::error!("failed to start server application"),
    };
}
