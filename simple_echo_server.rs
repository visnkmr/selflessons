// This is a simple TCP stream server in Rust that logs all requests to the console
// It listens for incoming connections on port 8080 and echoes back the received data
// It uses the standard library modules std::net and std::io

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    // Create a TCP listener on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on port 8080");

    // Loop over the incoming connections
    for stream in listener.incoming() {
        // Handle each connection in a separate thread
        std::thread::spawn(|| {
            // Get the TCP stream or print an error
            let mut stream = match stream {
                Ok(stream) => stream,
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                }
            };

            // Create a buffer to store the received data
            let mut buffer = [0; 1024];

            // Loop over the incoming data
            loop {
                // Read from the stream or break the loop
                let n = match stream.read(&mut buffer) {
                    Ok(n) if n == 0 => break,
                    Ok(n) => n,
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                };

                // Print the received data to the console
                println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

                // Write back the data to the stream or break the loop
                if let Err(e) = stream.write_all(&buffer[..n]) {
                    println!("Error: {}", e);
                    break;
                }
            }

            // Print a message when the connection is closed
            println!("Connection closed");
        });
    }
}