use std::io::prelude::*;
use tokio::net::TcpStream;

// const ECHO_SERVER_ADDRESS: &str = "localhost:1234";
const ECHO_SERVER_ADDRESS: &str = "127.0.0.1:1234";

#[tokio::main]
async fn main(){
    //connection
    println!("connecting to {}", ECHO_SERVER_ADDRESS);
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await{
        // connected
        println!("connected to echo server {}:{}", 
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );
        // // write a message
        // let message = "Hello World";
        // let _ = stream.write(message.as_bytes());
        // let _ = stream.flush(); 
        // print!("sent: {}", message);
        // // read the result
        
        // let mut buffer = [0;1024];
        // let len = stream.read(&mut buffer).unwrap();
        // let message = String::from_utf8_lossy(&buffer);
        // println!("received:{}", message);
    }
    else {
        println!("failed to connect to {}", ECHO_SERVER_ADDRESS);
    }
}