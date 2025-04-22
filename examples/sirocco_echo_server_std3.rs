use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::env::args;
use std::{thread, time::Duration};

// const
const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main(){
    // read arguments
    let delay = args().nth(1).unwrap_or_default().parse::<u64>().unwrap_or_default();
    println!("Delay: {}", delay);
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
        handle_connection(stream, delay);
    }
}

fn handle_connection(mut stream: TcpStream, delay: u64) {
    // read the buffer
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();
    let message =String::from_utf8_lossy(&buffer[..len]);
    println!("Received: {}", message);
    
    //delay
    thread::sleep(Duration::from_millis(delay));
    let _ = stream.write_all(message.as_bytes()).unwrap();
    println!("Sent: {}", message);
}

