use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

fn main(){
    //connection
    println!("connecting to {}", ECHO_SERVER_ADDRESS);
}