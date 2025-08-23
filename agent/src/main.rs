mod network;
use std::sync::Arc;
use std::time::Duration;

use network::{Client, MultiplexManager};

fn main() {
    let client = loop {
        match Client::new("127.0.0.1:1337") {
            Ok(client) => break client,
            Err(_) => {
                eprintln!("Failed to connect to server, retrying in 5 seconds..");
                std::thread::sleep(Duration::from_secs(5));
            }
        }
    };
    println!("Connected to server successfully!");

    let manager = Arc::new(MultiplexManager::new(client));

    let _receive_thread = manager.start();

    println!("Multiplex manager started, waiting for streams from server...");

    loop {
        match manager.accept_stream() {
            Ok(stream) => {
                println!("Stream opened by server: stream_id={}", stream.id());

                std::thread::spawn(move || {
                    loop {
                        match stream.receive() {
                            Ok(data) => {
                                if data.is_empty() {
                                    println!("Stream {} closed", stream.id());
                                    break;
                                }
                                println!(
                                    "Stream {}: received {} bytes: {:?}",
                                    stream.id(),
                                    data.len(),
                                    String::from_utf8_lossy(&data)
                                );

                                if let Err(e) = stream.send_bytes(&data) {
                                    eprintln!("Failed to send on stream {}: {}", stream.id(), e);
                                    break;
                                }
                                println!("Stream {}: sent echo", stream.id());
                            }
                            Err(e) => {
                                eprintln!("Stream {} receive error: {}", stream.id(), e);
                                break;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept stream: {}", e);
                break;
            }
        }
    }
}
