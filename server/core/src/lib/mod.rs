use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Connection {
    pub addr: Ipv4Addr,
    pub port: u16,
}

#[derive(Debug)]
pub struct ServerState {}
