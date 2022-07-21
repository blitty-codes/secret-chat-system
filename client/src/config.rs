use std::{
    net::SocketAddrV4,
    str::FromStr,
    path::PathBuf,
    fs,
};

use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct Configuration {
    nick: String,
    default_server: Option<SocketAddrV4>,
    server_list: Vec<SocketAddrV4>
}

impl Configuration {
    pub fn has_default_server(&self) -> bool {
        self.default_server.is_some()
    }

    pub fn get_default_server(&self) -> Option<SocketAddrV4> {
        self.default_server
    }

    pub fn get_nick(&self) -> String {
        self.nick.clone()
    }

    pub fn get_server_list(&self) -> Vec<SocketAddrV4> {
        self.server_list.clone()
    }

    pub fn server_list_len(&self) -> usize {
        self.server_list.len()
    }

    pub fn set_default_server(&mut self, position: usize) {
        self.default_server = Some(self.server_list[position]);
    }
}

pub fn default_configuration() -> String {
    let random = rand::random::<u8>();
    let conf = format!("nick: \"krispy {}\"\ndefault_server: null\nserver_list: []", random);

    conf
}

pub fn save_configuration(conf: &Configuration, path: PathBuf) {
    let conf = format!("nick: \"{}\"\ndefault_server: {}\nserver_list: {:?}",
        conf.get_nick(),
        conf.get_default_server().unwrap().to_string(),
        conf.get_server_list().to_vec()
    );

    fs::write(path, conf).expect("Unable to write file");
}

pub fn read_configuration(conf: String) -> Configuration {
    let yaml = YamlLoader::load_from_str(&conf).unwrap();

    let nick = yaml[0]["nick"].as_str().unwrap().clone();
    let default_server = yaml[0]["default_server"].as_str().unwrap_or_default().clone();
    let server_list = yaml[0]["server_list"].as_vec().unwrap().clone();

    let server_list: Vec<SocketAddrV4> = server_list   
        .iter()
        .map(|f| {
            let x = f.as_str().unwrap();
            return SocketAddrV4::from_str(&x.to_string()).unwrap();
        })
        .collect();

    let server_list = Vec::from(server_list);
    if default_server == String::from("") {
        return Configuration {
            nick: nick.to_string(),
            default_server: None,
            server_list
        };
    }

    Configuration {
        nick: nick.to_string(),
        default_server: Some(SocketAddrV4::from_str(&default_server).unwrap()),
        server_list
    }
}
