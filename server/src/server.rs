// use std::io::Read;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::iter::Zip;
use std::net::{ TcpListener, SocketAddr, TcpStream, SocketAddrV4, Ipv4Addr, IpAddr };
use std::time::Duration;


use rand::{Error, Rng};

use crate::custom_rsa::{ RSA, ClientRSA };
use crate::connection::ClientConnection;
// #[path = "./connection.rs"] mod connection;
// use crate::connection::Connection;

#[derive(Debug)]
pub enum ConnectionErrors {
	ConnectionFailed,
}


#[derive(Debug, Clone)]
pub struct Server {
	name: String,
	socket: SocketAddr,
	key: RSA,
	client_collection: HashMap<String, ClientConnection>,
}

impl Server {
	pub fn new(name: String, socket: SocketAddr) -> Server {
		Server {
			name,
			socket,
			key: RSA::new(),
			client_collection: HashMap::new(),
		}
	}

	pub fn connect(&self) -> TcpListener {
		if let Ok(conn) = TcpListener::bind(self.socket) {
			println!("Server {} - connected to {}", self.name, self.socket);
			conn
		} else {
			panic!("Couldn't connect to server...");
		}
	}

	// TODO: This is the .incoming() where we do the swap of cyphers.
	pub fn listen(&mut self) {
		let listener = self.connect();

		for stream in listener.incoming() {
			match &stream {
				Ok(stream) => {
					self.recv_string(stream);
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}
	}

	// TODO: Encrypt this
	fn buffer_it(&mut self, sender: &String, msg: &String) -> Result<(), ConnectionErrors> {
		for (name, client_connection) in &mut self.client_collection {
			println!("sender: {} - {}", sender, name);
			if sender == name { continue; }
			else {
				client_connection.buffer.push((sender.clone(), msg.clone()));
				println!("Mensajes para {}: {:?}", name, client_connection.buffer);
			}
		}
		Ok(())
	}

	fn fetch_it(&mut self, reciver: &String) -> String {
		match self.client_collection.get_mut(reciver) {
			Some(c) => {
				println!("Mensajes para {}: {:?}", reciver, c.buffer);
				let buff: Vec<(String, String)> = c.buffer
					.drain(..)
					.collect();
				let mut msg: String = buff
					.iter()
					.map(|b| format!("{} :=> {} - ", b.0.clone(), b.1.clone()))
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
		println!("----------\nConnection established with {}", stream.peer_addr().unwrap());
		// stream.set_read_timeout(Some(Duration::new(5, 500))).unwrap();
		// stream.set_write_timeout(Some(Duration::new(5, 500))).unwrap();
		
		let mut reader = BufReader::new(&mut stream);
		let mut str_buff: String = "".to_string();

		// TODO: HAY QUE HACERLO ASI, realizar las últimas modificaciones
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
				// here ipv4 is of type IpV4Addr
				let port = Server::assign_port();
				let tcp_sock = SocketAddrV4::new(ipv4, port);
				self.client_collection.insert(name.clone(), ClientConnection::new(tcp_sock, client_rsa.clone()));
	
				println!("> Added new collection");
				let hand = client_rsa.encrypt(&msg);
				let name = client_rsa.encrypt(&self.name.to_string().clone());
				let port = client_rsa.encrypt(&port.to_string());
	
				let name = base64::encode(name);
				let hand = base64::encode(hand);
				let port = base64::encode(port);
				// println!("{} -> {}", name, tcp_sock);
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

			println!("client name: {}", name);
			if self.client_collection.contains_key(name) {
				// println!("Decrypted: {} - {}", name, msg);
				// TODO: FORWARD MESSAGE TO OTHER CLIENTS
				//self.send_string(decrypted_msg)
				if msg == "fetch".to_string() {
					println!("Fetching...");
					let msg = self.fetch_it(&name.to_string());
					// println!("Fetched. Sending...");
					// println!("msg: {}", msg);
					writeln!(stream, "{}", msg).unwrap();
					println!("Sended.");
				} else {
					self.buffer_it(&name.to_string(), &msg.to_string()).unwrap();
				}
			}
		}

		println!("Done.\n----------");
	}
/*
	fn connect_handshake(&self, ip: SocketAddr) -> Result<TcpStream, ConnectionErrors> {
		if let Ok(conn) = TcpStream::connect(ip) {
			Ok(conn)
		} else {
			Err(ConnectionErrors::ConnectionFailed)
		}
	}

	pub fn send_handshake_id (&self, msg: String, ip: SocketAddr) {
		self.send_handshake(format!("{} - {}", self.name.to_string(), msg), ip);
	}

	// TODO: Send junk so that middleman is just like WTF
	// TODO: Asymetric encryption (hole) OwO
	fn send_handshake(&self, data: String, ip: SocketAddr) {
		// let data_mut =  format!("{}> {}", self.name, data.clone().as_str());
		// .write(&data_mut.as_bytes())
		match &mut self.connect_handshake(ip) {
			Ok(stream) => {
				println!("Sending: {}", data);

				stream.write(&data.as_bytes()).unwrap();
			}
			Err(e) => println!("Error: {:?}", e),
		}
	}
*/

	/*
	fn fordward_msg(&mut self, msg: String) {
		for (name, client_rsa) in self.client_collection.iter_mut() {
			let encrypted_msg = client_rsa.encrypt(&msg);
			self.send_string(format!("{} - {}", name, encrypted_msg));
		}
	}
	*/
}

