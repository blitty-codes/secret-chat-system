// use std::io::Read;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::net::{ TcpListener, SocketAddr, TcpStream, SocketAddrV4, IpAddr };

use rand::Rng;

use crate::custom_rsa::{ RSA, ClientRSA };
use crate::connection::ClientConnection;
use crate::logger::{Logger, self};

#[derive(Debug, Clone)]
pub struct Server {
	name: String,
	socket: SocketAddr,
	key: RSA,
	client_collection: HashMap<String, ClientConnection>,
	logger: Logger
}

impl Server {
	pub fn new(name: String, socket: SocketAddr, logger: Logger) -> Server {
		Server {
			name,
			socket,
			key: RSA::new(),
			client_collection: HashMap::new(),
			logger
		}
	}

	pub fn connect(&self) -> TcpListener {
		if let Ok(conn) = TcpListener::bind(self.socket) {
			let msg = format!("Server {} - started on {}", self.name, self.socket);
			println!("{}", msg);
			logger::write_log(&self.logger, msg);
			conn
		} else {
			logger::write_issue_log(&self.logger, "Couldn't connect to server...".to_owned());
			panic!();
		}
	}

	pub fn listen(&mut self) {
		let listener = self.connect();

		for stream in listener.incoming() {
			match &stream {
				Ok(stream) => {
					self.recv_string(stream);
				}
				Err(e) => {
					logger::write_issue_log(&self.logger, format!("Error: {}", e));
				}
			}
		}
	}

	fn buffer_it(&mut self, sender: &String, msg: &String) {
		for (name, client_connection) in &mut self.client_collection {
			if sender == name { continue; }
			else {
				let s = client_connection.rsa_key.encrypt(&sender.clone());
				let m = client_connection.rsa_key.encrypt(&msg.clone());
				client_connection.buffer.push((s, m));
			}
		}
	}

	fn fetch_it(&mut self, reciver: &String) -> String {
		match self.client_collection.get_mut(reciver) {
			Some(c) => {
				let buff: Vec<(Vec<u8>, Vec<u8>)> = c.buffer
					.drain(..)
					.collect();
				let mut msg: String = buff
					.iter()
					.map(|b| {
						let name = base64::encode(b.0.clone());
						let msg = base64::encode(b.1.clone());

						format!("{} :=> {} - ", name, msg)
					})
					.collect();
				
				msg += " - finished";

				msg
			},
			None => todo!(),
		}
	}

	fn assign_port() -> u16 {
		rand::thread_rng().gen_range(50000..65000)
	}

	fn recv_string(&mut self, mut stream: &TcpStream) {
		logger::write_log(&self.logger, format!("Connection established with {}", stream.peer_addr().unwrap()));
		
		let mut reader = BufReader::new(&mut stream);
		let mut str_buff: String = "".to_string();

		while !str_buff.contains("- finished") {
			reader.read_line(&mut str_buff).unwrap();			
		}

		let vec: Vec<&str> = str_buff.split(" - ").collect();

		let name = vec[0].to_string();
		let msg = vec[1].to_string();

		// println!("> {} --:: {}", name, msg);
		if msg == "HANDSHAKE--::" {
			let client_rsa = ClientRSA::new(vec[2].to_string());
			let ip = stream.peer_addr().unwrap().ip();
			if let IpAddr::V4(ipv4) = ip {
				// ipv4 is of type IpV4Addr
				let port = Server::assign_port();
				let tcp_sock = SocketAddrV4::new(ipv4, port);
				self.client_collection.insert(name.clone(), ClientConnection::new(tcp_sock, client_rsa.clone()));
	
				logger::write_log(&self.logger, "> Added new client to collection".to_owned());
				let hand = client_rsa.encrypt(&msg);
				let name = client_rsa.encrypt(&self.name.to_string().clone());
				let port = client_rsa.encrypt(&port.to_string());
	
				let name = base64::encode(name);
				let hand = base64::encode(hand);
				let port = base64::encode(port);

				let msg = format!("{} - {} - {} - {} - finished", name, port, hand, self.key.get_public_key());
				writeln!(stream, "{}", msg).unwrap();
			}
		} else {
			let name = base64::decode(name).unwrap();
			let msg = base64::decode(msg).unwrap();

			let msg = self.key.decrypt(&msg);
			let name = self.key.decrypt(&name);

			let name = std::str::from_utf8(&name).unwrap();
			let msg = std::str::from_utf8(&msg).unwrap();

			if self.client_collection.contains_key(name) {
				if msg == "fetch".to_string() {
					logger::write_log(&self.logger, format!("{} fetching...", stream.peer_addr().unwrap()));
					let msg = self.fetch_it(&name.to_string());
					writeln!(stream, "{}", msg).unwrap();
					logger::write_log(&self.logger, format!("{} messages sent", stream.peer_addr().unwrap()));
				} else {
					self.buffer_it(&name.to_string(), &msg.to_string());
				}
			}
		}
	}
}
