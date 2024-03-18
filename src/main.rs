mod config;

use std::io::stdin;
use std::net::UdpSocket;
use crate::config::Config;

fn main() {
    let config = Config::from_json("config.json".to_string());
    let server_address: String = get_server_address();
    listen(&server_address, &config);
}

fn get_server_address() -> String {
    println!("Enter server address (format xxx.xxx.xxx.xxx:port)");
    let mut server_address = String::new();
    stdin().read_line(&mut server_address).expect("Reading the server address from input");
    return server_address.trim().to_string();
}

fn listen(server_address: &str, config: &Config) {
    let serv_config = config;
    println!("listening on {}", server_address);
    let socket: UdpSocket = UdpSocket::bind(server_address).expect("Error while binding the socket.");

    loop {
        let mut message: String = String::new();
        let mut buf: [u8; 1024] = [0; 1024];
        let (num_bytes, src_addr) = socket.recv_from(&mut buf).expect("Error while receiving message");
        if !serv_config.addr_is_blacklisted(src_addr.to_string()) {
            message.push_str(std::str::from_utf8(&buf[..num_bytes]).expect("Invalid UTF-8 data"));

            let forwarding_addr_opt = serv_config.is_forwarded(src_addr.to_string());
            if forwarding_addr_opt.is_some() {
                let forwarding_addr = forwarding_addr_opt.unwrap();
                socket.send_to(message.as_bytes(), forwarding_addr).expect("Error forwarding message to another ip.");
            }
            println!("{}: {}", src_addr.to_string(), message);
        }
    }
}
