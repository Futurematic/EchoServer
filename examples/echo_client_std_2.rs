use std::io::{Read, Write};
use std::net::{TcpStream, Shutdown};

const ECHO_SERVER_ADDRESS: &str = "127.0.0.1:1234";

fn main() {
    println!("Connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        println!(
            "Connected from {}",
            stream.local_addr().unwrap()
        );

        let message = "Hello World";
        stream.write_all(message.as_bytes()).unwrap();
        stream.shutdown(Shutdown::Write).unwrap(); // <-- Wichtig

        println!("Sent: {}", message);

        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..len]);
        println!("Received: {}", response);
    } else {
        println!("Failed to connect to {}", ECHO_SERVER_ADDRESS);
    }
}
