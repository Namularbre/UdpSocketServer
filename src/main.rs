use std::net::UdpSocket;

use colored::Colorize;

use crate::config::Config;

mod config;

fn main() {
    let config = Config::from_json("config.json".to_string());
    listen(&config);
}

fn listen(config: &Config) {
    let serv_config = config;
    println!("{}: {}", "listening on".green(), serv_config.get_server_addr());
    let socket: UdpSocket = UdpSocket::bind(serv_config.get_server_addr()).expect("Error while binding the socket.");

    loop {
        let mut message: String = String::new();
        let mut buf: [u8; 1024] = [0; 1024];
        let (num_bytes, src_addr) = socket.recv_from(&mut buf).expect("Error while receiving message");
        if !serv_config.addr_is_blacklisted(src_addr.to_owned().to_string()) {
            message.push_str(std::str::from_utf8(&buf[..num_bytes]).expect("Invalid UTF-8 data"));

            let forwarding_addr_opt = serv_config.is_forwarded(src_addr.to_string());
            if forwarding_addr_opt.is_some() {
                let forwarding_addr = forwarding_addr_opt.unwrap();
                socket.send_to(message.as_bytes(), forwarding_addr).expect("Error forwarding message to another ip.");
            }
            println!("{}: {}", src_addr.to_string().green(), message);
        }
    }
}
