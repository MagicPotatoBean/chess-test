mod chess;
mod networking;
use std::net::{TcpListener, TcpStream};
fn main() {
    println!("");
    let client = TcpStream::connect("127.0.0.1:8080");
    if client.is_ok() {

    } else {
        println!("Failed to connect, error message: {}", client.expect_err("Expected error, got recieved ok"));
    }
}