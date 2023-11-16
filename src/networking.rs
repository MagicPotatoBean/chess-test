use std::{
    net::{TcpListener, TcpStream},
    thread,
};

pub fn try_connect(ip_and_port: String) -> Result<TcpStream, std::io::Error> {
    TcpStream::connect(ip_and_port)
}
pub fn host_server(ip_and_port: String) -> () {
    let listener: TcpListener = TcpListener::bind(ip_and_port).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => {
                println!("Connection request from: {}", tcp_stream.peer_addr());
                println!("Would you like to accept this request? [y/n]: ");
                let mut line = String::default();

            },
            Err(_) => {

            },
        }
    }
}
