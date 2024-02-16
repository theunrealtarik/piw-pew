#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod server;

use std::{net::Ipv4Addr, sync::Arc, thread};

use egui::mutex::Mutex;
use env_logger;

use gui::Window;
use server::Server;

use lib::*;

fn main() {
    env_logger::init();

    let connection = Arc::new(Connection {
        addr: Ipv4Addr::new(0, 0, 0, 0),
        port: 6969,
    });
    let state = Arc::new(Mutex::new(ServerState {}));

    thread::park();
}
