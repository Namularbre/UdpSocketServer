use std::io::stdin;
use std::net::UdpSocket;

fn main() {
    let server_address: String = get_server_address();
    listen(&server_address);
}

fn get_server_address() -> String {
    println!("Enter server address (format xxx.xxx.xxx.xxx:port)");
    let mut server_address = String::new();
    stdin().read_line(&mut server_address).expect("Reading the server address from input");
    return server_address.trim().to_string();
}

fn listen(server_address: &str) {
    println!("listening on {}", server_address);
    loop {
        let mut message: String = String::new();
        let mut buf: [u8; 1024] = [0; 1024];
        let socket: UdpSocket = UdpSocket::bind(server_address).expect("Error while binding the socket.");
        let (num_bytes, src_addr) = socket.recv_from(&mut buf).expect("Error while receiving message");
        message.push_str(std::str::from_utf8(&buf[..num_bytes]).expect("Invalid UTF-8 data"));

        println!("{}:{}", src_addr.to_string(), message);

        if message.trim() == "!STOP" {
            return;
        }
    }
}
