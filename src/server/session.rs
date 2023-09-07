use tokio::net::TcpStream;

use std::net::SocketAddr;

use super::telnet::Client;

pub struct Session {
    pub client: Client,
    pub addr: SocketAddr,
}

impl Session {
    pub fn new(socket: TcpStream, addr: SocketAddr) -> Self {
        Self {
            client: Client::new(socket),
            addr,
        }
    }
}
