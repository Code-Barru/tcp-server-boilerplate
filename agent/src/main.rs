mod network;

use std::thread;
use std::time::Duration;

use network::{Client, dispatch_frame};

fn main() {
    loop {
        let client = loop {
            match Client::new("127.0.0.1:1337") {
                Ok(client) => break client,
                Err(_) => {
                    eprintln!("Failed to connect to server, retrying in 5 seconds..");
                    thread::sleep(Duration::from_secs(5));
                }
            }
        };
        println!("Connected to server successfully!");

        let (mut reader, writer) = client.split();

        let receiver_thread = thread::spawn(move || {
            loop {
                match reader.receive_frame() {
                    Ok(frame) => {
                        println!("Received frame: {:?}", frame.packet_type);
                        let responses = dispatch_frame(&frame);

                        for response in responses {
                            if let Err(e) = writer.send_frame(&response) {
                                eprintln!("Failed to send response: {:?}", e);
                                return;
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("Failed to receive frame, connection may be down.");
                        break;
                    }
                }
            }
        });

        let _ = receiver_thread.join();
        thread::sleep(Duration::from_secs(5));
    }
}
