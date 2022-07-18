use std::borrow::{Borrow, BorrowMut};
use std::net::{ TcpStream, SocketAddr, TcpListener, Ipv4Addr, SocketAddrV4 };
use std::io::{prelude::*, BufReader};
use std::time::Duration;

use crate::network::custom_rsa::{ RSA, ServerRSA };
use crate::network::errors::*;
use crate::ui::input::ChatMessages;

#[derive(Debug, Clone)]
pub struct Connection {
	pub socket: SocketAddrV4,
	pub recv_socket: Option<SocketAddrV4>
}

#[derive(Debug, Clone)]
pub struct Client {
	name: String,
	pub stream: Connection,
	key: RSA,
	server_key: Option<ServerRSA>,
}

impl Client {
	pub fn new(name: String, stream: Option<Connection>) -> Client {
		let sock =  match stream {
			Some(s) => s,
			None => Connection {
				socket: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080),
				recv_socket: None
			},
		};

		Client {
			name,
			stream: sock,
			key: RSA::new(),
			server_key: None,
		}
	}

	pub fn get_name(&self) -> String {
		self.name.clone()
	}

	pub fn first_hand_shake(&mut self) {
		// handshake
		let public_key = self.key.get_public_key();
		self.send_string_id(format!("{} - {}", "HANDSHAKE--::".to_string(), public_key));
	}

	fn connect(&self) -> Result<TcpStream, ConnectionErrors> {
		if let Ok(conn) = TcpStream::connect(self.stream.socket) {
			Ok(conn)
		} else {
			Err(ConnectionErrors::ConnectionFailed)
		}
	}

	fn read_stream(&self, stream: &mut TcpStream) -> String {
		let mut reader = BufReader::new(stream);
		let mut str_buff: String = "".to_string();
		
		// Server response
		while !str_buff.contains("- finished") {
			reader.read_line(&mut str_buff).unwrap();			
		}

		str_buff
	}

	pub fn send_string_id(&mut self, msg: String) {
		match &mut self.connect() {
			Ok(stream) => {
				self.send_string(stream, format!("{} - {} - finished", self.get_name().to_string(), msg));
			}
			Err(_) => {}
		}
	}

	pub fn fetch_msg(&mut self) -> Result<Vec<(String, String)>, ConnectionErrors> {
		match &mut self.connect() {
			Ok(stream) => {
				// stream.set_nonblocking(true).unwrap();
				// println!("Sending---");
				self.send_string(stream, format!("{} - {} - finished", self.get_name().to_string(), "fetch".to_string()));
				// println!("reading messages...");
				let messages = self.read_stream(stream);
				// println!("messages: {}", messages);

				let mut msg: Vec<String> = messages
					.split(" - ")
					.map(| m | String::from(m))
					.collect();

				if msg.len() <= 2 {
					msg.drain(0..msg.len());

					Ok(Vec::new())
				} else {
					// check for "Buenos días a todos!! -  - finished"
					let msg = msg[0..msg.len()-2].to_vec();
	
					let msg: Vec<(String, String)> = msg
						.iter()
						.map(| msg | {
							let c: Vec<&str> = msg
								.split(" :=> ")
								.collect();
							return (String::from(c[0]), String::from(c[1]));
						})
						.collect();
					
					Ok(msg)
				}

			}
			Err(e) => {
				panic!("Error on fetching: {:?}", e);
			},
		}
	}

	// TODO: Send junk so that middleman is just like WTF
	// TODO: Asymetric encryption (hole) OwO
	fn send_string(&mut self, stream: &mut TcpStream, mut data: String) {
		// let data_mut =  format!("{}> {}", self.name, data.clone().as_str());
		// .write(&data_mut.as_bytes())
				// stream.set_nonblocking(true).unwrap();

		//  name and msg are encrypted
		match &self.server_key {
			Some(key) => {
				let mut vec: Vec<&str> = data.split(" - ").collect();

				let name_enc = key.encrypt(vec[0]);
				let msg_enc = key.encrypt(vec[1]);

				let b64_name = base64::encode(name_enc).to_string();
				let b64_msg = base64::encode(msg_enc).to_string();

				vec[0] = &b64_name;
				vec[1] = &b64_msg;

				data = format!("{} - {} - finished", vec[0], vec[1]);
				
				writeln!(stream, "{}", data.to_string()).unwrap();
			},
			// handshake
			None => {
				writeln!(stream, "{}", data.to_string()).unwrap();
				let msg = self.read_stream(stream);
				let vec: Vec<&str> = msg.split(" - ").collect();

				// let name_enc = base64::decode(vec[0]).unwrap();
				let port_enc = base64::decode(vec[1]).unwrap();
				let msg_enc = base64::decode(vec[2]).unwrap();

				// let name = self.key.decrypt(&name_enc);
				let port = self.key.decrypt(&port_enc);
				let msg = self.key.decrypt(&msg_enc);

				// let name = std::str::from_utf8(&name).unwrap();
				let port = std::str::from_utf8(&port).unwrap();
				let msg = std::str::from_utf8(&msg).unwrap();

				// println!("> {} --:: {}", name, msg);
				// double check
				if msg == "HANDSHAKE--::" {
					let server_rsa = ServerRSA::new(vec[3].to_string());
					self.server_key = Some(server_rsa);
					self.stream.recv_socket = Some(SocketAddrV4::new(
						self.stream.socket.ip().clone(),
						port.parse::<u16>().unwrap()
					));
					// println!("> Server {}: {:#?}", name, self.server_key);
					// println!("{} - {} - {}", self.name.to_string(), "HANDSHAKE--::".to_string(), self.key.get_public_key().to_string());
					//self.send_string(format!("{} - {}", "HANDSHAKE--::".to_string(), client_rsa.get_public_key()));
				}
			},
		}

		// stream.set_read_timeout(Some(Duration::new(10, 500))).unwrap();
		// stream.set_write_timeout(Some(Duration::new(10, 500))).unwrap();

		// println!("Done.");

		// match &mut self.connect() {
		// 	Ok(stream) => {
		// 		let mut str_buff: String = "".to_string();
		// 		println!("stf_buff: {}", str_buff);
		// 		let i = stream.read_to_string(&mut str_buff)
		// 				.unwrap();
		// 		println!("i: {}", i);
		// 				//.unwrap_or_else(| err | panic!("Error:\n {}", err));

		// 		let vec: Vec<&str> = str_buff.split(" - ").collect();

		// 		let name = vec[0].to_string();
		// 		let msg = vec[1].to_string();

		// 		println!("> {} --:: {}", name, msg);
		// 		if msg == "HANDSHAKE--::" {
		// 			let client_rsa = ServerRSA::new(vec[2].to_string());
		// 			self.server_key = Some(client_rsa);
		// 			println!("> Server {}: {:#?}", name, self.server_key);
		// 			println!("{} - {} - {}", self.name.to_string(), "HANDSHAKE--::".to_string(), self.key.get_public_key().to_string());
		// 			//self.send_string(format!("{} - {}", "HANDSHAKE--::".to_string(), client_rsa.get_public_key()));
		// 		}
		// 	},
		// 	Err(_) => {},
		// }
	}
}
