use std::net::TcpListener;

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
        let _stream = stream.unwrap();
        println!("Connection established from");
    }
}

