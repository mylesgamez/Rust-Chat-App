// main.rs

use std::net::{TcpStream};
use std::io::{stdin, Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:8080") {
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
        Ok(mut stream) => {
            println!("Successfully connected to server in port 8080");

            let mut data = [0 as u8; 50]; // using 50 byte buffer
            while match stdin().read(&mut data) {
                Ok(size) => {
                    stream.write(&data[0..size]).unwrap();
                    stream.read(&mut data).unwrap();
                    println!("Received: {}", from_utf8(&data).unwrap());
                    true
                },
                Err(_) => {
                    println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                    false
                }
            } {}
        }
    }
}

