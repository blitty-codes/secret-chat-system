use std::net::{ TcpStream, Ipv4Addr, SocketAddrV4 };
use std::str::{self, from_utf8};
use std::io::{prelude::*, BufReader};

use crate::network::custom_rsa::{ RSA, ServerRSA };
use crate::network::errors::*;

#[derive(Debug, Clone)]
pub struct Client {
	name: String,
	stream: SocketAddrV4,
	key: RSA,
	server_key: Option<ServerRSA>,
}

impl Client {
	pub fn new(name: String, stream: Option<SocketAddrV4>) -> Client {
		let sock =  match stream {
			Some(s) => s,
			None => SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080),
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

	pub fn first_hand_shake(&mut self) -> Result<(), ConnectionErrors> {
		// handshake
		let public_key = self.key.get_public_key();
		match self.send_string_id(format!("{} - {}", "HANDSHAKE--::".to_string(), public_key)) {
			Ok(_) => Ok(()),
			Err(e) => return Err(e),
		}
	}

	fn connect(&self) -> Result<TcpStream, ConnectionErrors> {
		if let Ok(conn) = TcpStream::connect(self.stream) {
			Ok(conn)
		} else {
			return Err(ConnectionErrors::FailedHandshake);
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

	pub fn send_string_id(&mut self, msg: String) -> Result<(), ConnectionErrors> {
		match &mut self.connect() {
			Ok(stream) => {
				self.send_string(stream, format!("{} - {} - finished", self.get_name().to_string(), msg));
				Ok(())
			}
			Err(_) => {
				Err(ConnectionErrors::FailedSendString)
			}
		}
	}

	pub fn fetch_msg(&mut self) -> Result<Vec<(String, String)>, ConnectionErrors> {
		match &mut self.connect() {
			Ok(stream) => {
				self.send_string(stream, format!("{} - {} - finished", self.get_name().to_string(), "fetch".to_string()));
				let messages = self.read_stream(stream);

				let mut msg: Vec<String> = messages
					.split(" - ")
					.map(| m | String::from(m))
					.collect();

				if msg.len() <= 2 {
					msg.drain(0..msg.len());

					Ok(Vec::new())
				} else {
					let msg = msg[0..msg.len()-2].to_vec();
	
					let msg: Vec<(String, String)> = msg
						.iter()
						.map(| msg | {
							let c: Vec<&str> = msg
								.split(" :=> ")
								.collect();
							
							let client = base64::decode(c[0].to_string()).unwrap();
							let msg = base64::decode(c[1].to_string()).unwrap();

							let client = self.key.decrypt(&client);
							let msg = self.key.decrypt(&msg);

							let client = from_utf8(&client).unwrap();
							let msg = from_utf8(&msg).unwrap();

							return (client.to_string(), msg.to_string());
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

				let msg_enc = base64::decode(vec[2]).unwrap();
				let msg = self.key.decrypt(&msg_enc);
				let msg = std::str::from_utf8(&msg).unwrap();

				// double check
				if msg == "HANDSHAKE--::" {
					let server_rsa = ServerRSA::new(vec[3].to_string());
					self.server_key = Some(server_rsa);
				}
			},
		}

		// stream.set_read_timeout(Some(Duration::new(10, 500))).unwrap();
		// stream.set_write_timeout(Some(Duration::new(10, 500))).unwrap();
	}
}
