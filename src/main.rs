use std::net::UdpSocket;
use std::io::stdin;

fn main() {
    let server_address: String = get_server_address();
    listen(server_address);
}

fn get_server_address() -> String {
    println!("Enter server address (format xxx.xxx.xxx.xxx:port)");
    let mut server_address = String::new();
    stdin().read_line(&mut server_address).expect("Reading the server address from input");
    server_address = String::from(server_address.trim());
    return server_address;
}

fn listen(server_address: String) {
    println!("listening on {}", server_address);
    loop {
        let mut message: String = String::new();
        let mut buf: [u8; 1024] = [0; 1024];
        let socket: UdpSocket = UdpSocket::bind(server_address.clone()).expect("Error while binding the socket.");
        let (num_bytes, src_addr) = socket.recv_from(&mut buf).expect("Error while receiving message");
        message.push_str(std::str::from_utf8(&buf[..num_bytes]).expect("Invalid UTF-8 data"));
        println!("{}:{}", src_addr.to_string(), message);

        if message.trim() == "!STOP" {
            return;
        }
    }
}