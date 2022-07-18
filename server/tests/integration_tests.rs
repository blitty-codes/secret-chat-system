#[cfg(test)]
mod tests {
	use std::thread;
	use std::io::prelude::*;
	use std::net::{ Ipv4Addr, SocketAddr, TcpStream };

	const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1); 
	const PORT: u16 = 8080;

	fn client_configuration () -> TcpStream {
		// Client::new("Async Rusty".to_string(), SocketAddr::from((ADDR, PORT))).unwrap()
		let socket = SocketAddr::from((ADDR, PORT));
		TcpStream::connect(socket).unwrap()
	}

	#[test]
	pub fn test () -> Result<(), String> {
		println!("Number of clients: 5");
		for i in 1..5 {
			let c = thread::Builder::new().name(format!("Client {}", i));
			let client = c.spawn(move || {
				let mut client = client_configuration();
				let msg = format!("Client{} - Hello from client {:#?}", i, i);
				println!("{}", msg);
				client.write(msg.as_bytes()).unwrap();
			}).unwrap();
			client.join().unwrap();
		}

		println!("current thread id: {:?}", thread::current().id());

		Ok(())
	}
}
