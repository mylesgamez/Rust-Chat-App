// main.rs

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    println!("Incoming connection from: {}", stream.peer_addr()?);

    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        println!("Message from client: {}", String::from_utf8_lossy(&buf));
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                        .unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            },
        }
    }
    Ok(())
}

