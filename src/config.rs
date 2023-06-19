use std::env;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

pub struct Config {
    pub socket_addr: SocketAddr,
}

pub fn get() -> Config {
    let host: String = env::var("HOST").unwrap_or(String::from("0.0.0.0"));

    let port: u16 = env::var("PORT")
        .unwrap_or(String::from("1"))
        .parse()
        .unwrap();

    let ip = match host.parse::<Ipv4Addr>() {
        Ok(ip) => ip,
        Err(_) => {
            panic!("Invalid host address");
        }
    };

    let socket_addr = SocketAddr::V4(SocketAddrV4::new(ip, port));

    Config { socket_addr }
}
