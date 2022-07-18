use std::net::{TcpStream, SocketAddrV4};

use crate::custom_rsa::ClientRSA;

#[derive(Debug, Clone)]
pub struct ClientConnection {
	pub stream: SocketAddrV4,
	pub rsa_key: ClientRSA,
	pub buffer: Vec<(String, String)>
}

impl ClientConnection {
	pub fn new(stream: SocketAddrV4, rsa_key: ClientRSA) -> ClientConnection {
		ClientConnection {
			stream,
			rsa_key,
			buffer: Vec::new()
		}
	}
}
