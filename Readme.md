Echo Client / Server in Rust - YT Tutorial 

From Zero to Async Hero with Rust’s Tokio

Chris Hay kann man sehr gut zuhören und verstehen

Sein Projekt ist die Realisierung eines TCP Echo Client/Servers.

Vorbereitend hat er Echo client als auch Echo server mit socat realsiert.
Damit hat man einerseits ein Grundverständniss der Funktion und andererseits zwei Rust unabhängige Tools zum testen !.
Ich habe netcat verwendet.
Sowohl Server als auch Client lassen sich damit realisieren.

Leider kann der netcat server keine Nachrichteninhalte anzeigen sondern nur Verbinsungsstati.
Daher habe ich mir mit ChatGPT einen Powershell basierten Echo Server schreiben lassen


Die Rust Umsetzung erfolgt in zwei Schritten. Zuerst mit standard Rust crate und dann als asynchroner Echo Client mit Tokio!.

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


Obigen code erweitere ich um die Write / Read Funktionen und muss auch, entgegen dem Beispiel aus dem Video, nach dem stream.write_all auch die stream.shutdown Methode einbauen
echo_client_tokio_simple3.rs


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



Ganz generell ist die Funktionalität so wie bei dem Echo server mit dem standard crate.
In der Echo Client  Anwendung hat async / tokio natürlich keinen großen Sinn da es keine ‘parallele’ Aufgabe zu lösen gilt. Zumindest nicht in dieser Konstellation.

Das wird anders beim Echo Server
Dieser wird im Video erstmal ebenfalls mit dem standard Funktionen realisiert.
Nach einigen Iterationen sieht dieser so aus.
sirocco_echo_server_std3.rs

```rust
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
```


Dieser läuft erstmal wie erwartet. Also er horcht tcp mässig lokal auf Port 8000.
Um das eigentliche Problem zu verdeutlichen wurde in ibigem code ein delay eingebaut, welches beim Aufruf übergeben werden kann, also so
cargo run --example sirocco_echo_server_std3 5000
Dieses delay sorgt dafür das in der handle_connection Funktion nach dem Empfangen der nachricht von einem Client, um dieses delay gewartet wird (sleep), bevor die Nachricht zurück gesendet wird. Dies soll im Prinzip verdeutlichen das zwischen Empfang und Senden, immer eine Verzögerung entsteht, je nachdem wie Aufwändig z.B. die Verarbeitung einer Nachricht ist.

Damit wird auch folgendes Problem offensichtlich, nämlich wenn wie unten dargestellt zwei Client, in diesem Fall unser Rust Echo Client (mitte), der zuerst sendet und unser netcat client (unten) der sofort nach dem rust echo client sendet.
Der Echo Server emofängt sofort ‘Hello World’ vom Echo Client. Wartet nun 5 Sekunden, in dem er für die netcat Anforderung nicht zur Verfügung steht.
Sendet dann die Antwort an den rust Echo client und verarbeitet erst dann die Echoanforderung von netcat. Die er dann wiederum nach 5 Sek. beantwortet.



Es wäre also prinzipiell schön wenn unser Echo Server, alle Anforderungen, direkt und parallel beantworten könnte.

Im YT Video wird nun ein weiterer Server namens Karin ins Spiel gebracht.
Dieser fungiert als ‘man-in-the-middle’ bzw als Relay-Server für den echo-server.
Er empfängt auf Port 8001 Nachrichten, leitet diese an unseren (parallel laufenden) Rust Standard Echo-Server weiter, empfägnt dann von diesem (nach wartezeit) die Antwort und antwortet dann seinerseits dem ursprünglichen  Client.


karin_std.rs
call_sirocco(message: String)  entspricht unserem standard echo-server also kommunikation auf Port 8000. Die Nachricht die er als echo-client an sirocco sendet bekommt er via message Funktionsparameter.
Diese akt. version ist aufgrund der standard Funktionen noch nicht async ready. Die Relaying eigenschaft ist aber ok.

```rust
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
       // let _  stream.shutdown(Shutdown::Write).unwrap(); // <-- Wichtig
       let _ =  stream.shutdown(Shutdown::Write).unwrap(); // <-- Wichtig
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
```



Nun folgt die um tokio / async erweiterte Version.
Dabei ist einiges zu beachten:
Neben den aus dem Tokio Echo-Client bereits bekannten Änderungen und Codeumbauten müssen wir beachten das die for Schleife durch eine Loop ersetzt wird, den tokio kennt keine Incoming  Eigenschaft for stream in listener.incoming()
Weiterhin änder sich das Verhalten trotz aller async Umrüstungen erst dann wenn in der incomning loop ‘gespawned’ wird
        tokio::spawn(async move {
            handle_connection(stream).await;
Erst hiermit bekommt die loop die Möglichkeit die handle_connection Funktion Nebenläufig auszuführen !

```rust
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
       // let _  stream.shutdown(Shutdown::Write).unwrap(); // <-- Wichtig
       // let _ =  stream.flush.await.unwrap(); // <-- Wichtig
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
```
Damit sieht man nun das der echo-server (links) der mit delay 10 sek. gestartet wurde läuft.
Karin läuft und nimmt alle Anforderungen sofort an.
Dies sieht man daran das das Hello World vom rust Echo-Client angenommen worden ist, während die Beantwortung der netcat Anfrage durch den Echo-Server noch läuft.

