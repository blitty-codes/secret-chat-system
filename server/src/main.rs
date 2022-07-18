use std::net::{ Ipv4Addr, SocketAddr };

mod server;
mod connection;
mod custom_rsa;
use crate::server::Server;

const PORT : u16 = 8080;
const ADDR : Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1); 

fn main() {
    // custom_rsa::RSA::new().print();

    let socket = SocketAddr::from((ADDR, PORT));

    let mut s = Server::new("Rusty".to_string(), socket);
    s.listen();
}
