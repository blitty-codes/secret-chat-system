use std::net::SocketAddrV4;

use crate::custom_rsa::ClientRSA;

#[derive(Debug, Clone)]
pub struct ClientConnection {
	pub stream: SocketAddrV4,
	pub rsa_key: ClientRSA,
	pub buffer: Vec<(Vec<u8>, Vec<u8>)>
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
