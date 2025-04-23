use std::io::{Read, Write};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt}; 
use std::net::Shutdown;

// const
const KARIN_SERVER_ADDRESS: &str = "127.0.0.1:8001";
const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main(){
    // starting
    println!("Karin staring {}", KARIN_SERVER_ADDRESS);

    // bind
    let listener = TcpListener::bind(KARIN_SERVER_ADDRESS).await.unwrap();
    // start the server
    println!("Karin echo server started on {}", KARIN_SERVER_ADDRESS);
   
    // accept incoming connections
    loop {
        let (stream,_) = listener.accept().await.unwrap();
        // spawning thread to make async really work
        tokio::spawn(async move {
            handle_connection(stream).await;
            // println!("Connection established from {}", stream.peer_addr().unwrap());
        });
        // handle the connection in a separate thread
        // handle_connection(stream).await;
        // println!("Connection established from {}", stream.peer_addr().unwrap());
    }
}

async fn handle_connection(mut stream: TcpStream) {
    // read the buffer
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).await.unwrap();
    let message =String::from_utf8_lossy(&buffer[..len]);
    println!("Received: {}", message);
    
    // call sirocco
    let sirocco_message = call_sirocco(message.to_owned().to_string()).await;
    let output = format!("Sorocco says : {}", sirocco_message);
    // write message
    let _ = stream.write_all(output.as_bytes()).await.unwrap();
    println!("Sent: {}", output);
}

async fn call_sirocco(message: String) -> String {
    // connection to Sirocco server
    println!("Connecting to sirocco: {}", SIROCCO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(SIROCCO_SERVER_ADDRESS).await {
        println!(
            "Connected to sirocco {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

       let _ = stream.write_all(message.as_bytes()).await.unwrap();
       // let _  stream.shutdown(Shutdown::Write).unwrap(); // <-- Wichtig
       // let _ =  stream.flush.await.unwrap(); // <-- Wichtig
        println!("Sent to sirocco: {}", message);

        // read the result
        // and shutdown the read channel
        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer[..len]);
        println!("Received from sirocco: {}", message);

        return message.to_owned().to_string();
    } else {
        println!("Failed to connect to {}", SIROCCO_SERVER_ADDRESS);
        return message.to_owned().to_string();
    }
}