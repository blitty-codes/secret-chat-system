use std::{
    net::{
        Ipv4Addr,
        SocketAddr
    },
    fs,
    io::Write, env
};

mod server;
mod connection;
mod custom_rsa;
mod logger;
mod config;
use crate::server::Server;

const PORT : u16 = 8080;
const ADDR : Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1); 

const CONNECTION_LOGGER: &str = "logger";
const CONF_FILE: &str = "s-conf.yaml";

fn main() -> Result<(), std::io::Error>{
    let path = env::current_dir()?;
    let path = path.join(CONF_FILE);

    if !path.exists() {
        // create default configuration
        let conf = config::default_configuration();
        let mut file = fs::File::create(CONF_FILE).unwrap();
        file.write_all(conf.as_bytes()).unwrap();
    }

    let conf = fs::read_to_string(path.clone()).unwrap();
    let conf = config::read_configuration(conf);

    let logger = logger::create_log(CONNECTION_LOGGER.to_string()).unwrap();
    
    let socket = SocketAddr::from((conf.get_ip(), conf.get_port()));
    
    let mut s = Server::new("Rusty".to_string(), socket, logger.clone());

    s.listen();

    Ok(())
}
