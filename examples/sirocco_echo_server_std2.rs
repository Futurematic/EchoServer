use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// const
const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main(){
    // start the server
    println!("Sirocco echo server startet on {}", SIROCCO_SERVER_ADDRESS);
    // bind
    let listener = TcpListener::bind(SIROCCO_SERVER_ADDRESS).unwrap();
    println!("Server listening on {}", SIROCCO_SERVER_ADDRESS);
    // accept incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established from");
        // handle the connection in a separate thread
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // read the buffer
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();
    let message =String::from_utf8_lossy(&buffer[..len]);
    println!("Received: {}", message);
    // write the buffer back to the client
    let _ = stream.write_all(message.as_bytes()).unwrap();
    println!("Sent: {}", message);
}

