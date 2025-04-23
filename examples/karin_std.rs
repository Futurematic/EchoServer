use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};

// const
const KARIN_SERVER_ADDRESS: &str = "127.0.0.1:8001";
const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main(){
    // starting
    println!("Karin staring {}", KARIN_SERVER_ADDRESS);

    // bind
    let listener = TcpListener::bind(KARIN_SERVER_ADDRESS).unwrap();
    // start the server
    println!("Karin echo server started on {}", KARIN_SERVER_ADDRESS);
   
    // accept incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
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
    
    // call sirocco
    let sirocco_message = call_sirocco(message.to_owned().to_string());
    let output = format!("Sirocco says : {}", sirocco_message);
    // write message
    let _ = stream.write_all(output.as_bytes()).unwrap();
    println!("Sent: {}", output);
}

fn call_sirocco(message: String) -> String {
    // connection to Sirocco server
    println!("Connecting to sirocco: {}", SIROCCO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(SIROCCO_SERVER_ADDRESS) {
        println!(
            "Connected to sirocco {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

       let _ = stream.write_all(message.as_bytes()).unwrap();
       // let _  stream.shutdown(Shutdown::Write).unwrap(); // <-- Wichtig
       let _ =  stream.shutdown(Shutdown::Write).unwrap(); // <-- Wichtig
        println!("Sent to sirocco: {}", message);

        // read the result
        // and shutdown the read channel
        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..len]);
        println!("Received from sirocco: {}", message);

        return message.to_owned().to_string();
    } else {
        println!("Failed to connect to {}", SIROCCO_SERVER_ADDRESS);
        return message.to_owned().to_string();
    }
}