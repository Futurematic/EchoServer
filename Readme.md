Hier die überarbeitete Version aus dem YT Video
Ich musste  ein shutdown::Write einbauen
stream.shutdown(Shutdown::Write).unwrap(); // <-- Wichtig
das es funktioniert hat.
Bin mir nicht vollkommen im Klaren warum.
Denke das hängt mit dem Beenden einer echo server session zusammen.
Auf jeden Fall funktioniert so erstmal der Echo Client mit Rust standard crate

```rust
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
```

Nun wird der Echo client asynchron mit der tokio Erweiterung realisiert
cargo.toml wird um tokio erweitert um das entspr. crate einzubinden

```toml
[package]
name = "echoserver"
version = "0.1.0"
edition = "2024"

[dependencies]
tokio = {version = "1", features = ["full"]}
```

Ausgehend von dem oben realisiertem rust echo client, wird nun der client asynchron implementiert.
Dazu wird der Standard TCPStream gegen den von tokio ausgetauscht
Die main Funktion bekommt das tokio:main Makro und wird als async gekennzeichnet !.
Die TCPStream Funktionen erhalten die .await eigenschaft.
Damit wird das blockieren des netzwerk stack verhindert und asynchron auf eine Antwort gewartet.
Hier erstmal nur die sehr einfache Implementierung ohne versenden von Nachrichten.

```rust
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
```

```rust
// use std::io::prelude::*;
use tokio::io::{AsyncWriteExt,AsyncReadExt};
use tokio::{net::TcpStream};

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
        // write a message
        let message = "Hello World";
        let _ = stream.write_all(message.as_bytes()).await;
        // let _ = stream.flush(); 
        print!("sent: {}", message);
        // read the result
        
        let mut buffer = [0;1024];
        let len = stream.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer);
        println!("received:{}", message);
    }
    else {
        println!("failed to connect to {}", ECHO_SERVER_ADDRESS);
    }
}
```