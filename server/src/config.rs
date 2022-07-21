use std::net::Ipv4Addr;
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct Configuration {
    nick: String,
    ip: Ipv4Addr,
    port: u16
}

impl Configuration {
    pub fn get_ip(&self) -> Ipv4Addr {
        self.ip
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_nick(&self) -> String {
        self.nick.clone()
    }
}

pub fn default_configuration() -> String {
    let random = rand::random::<u8>();
    let conf = format!("nick: \"krispy {}\"\nip: 127.0.0.1\nport: 9009", random);

    conf
}

pub fn read_configuration(conf: String) -> Configuration {
    let yaml = YamlLoader::load_from_str(&conf).unwrap();

    let nick = yaml[0]["nick"].as_str().unwrap().clone();
    let ip = yaml[0]["ip"].as_str().unwrap().clone();
    let port = yaml[0]["port"].as_i64().unwrap().clone();

    if ip == String::from("") {
        return Configuration {
            nick: nick.to_string(),
            ip: Ipv4Addr::new(127, 0, 0, 1),
            port: port.try_into().unwrap()
        };
    }

    Configuration {
        nick: nick.to_string(),
        ip: ip.parse::<Ipv4Addr>().unwrap(),
        port: port.try_into().unwrap()
    }
}
