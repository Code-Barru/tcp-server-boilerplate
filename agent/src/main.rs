mod network;
use std::time::Duration;

use network::Client;

fn main() {
    loop {
        let mut client = loop {
            match Client::new("127.0.0.1:1337") {
                Ok(client) => break client,
                Err(_) => {
                    eprintln!("Failed to connect to server, retrying in 5 seconds..");
                    std::thread::sleep(Duration::from_secs(5));
                }
            }
        };
        println!("Connected to server successfully!");
        let mut counter = 0;
        loop {
            if counter >= 5 {
                let _ = client.send(&[]);
                std::thread::sleep(Duration::from_secs(1));
                std::process::exit(0);
            }

            let res = client.send(b"Hello from client!");
            if let Err(_) = res {
                eprintln!("Failed to send message, server may be down. Reconnecting in 5s...");
                break;
            }

            let response = match client.receive() {
                Ok(response) => response,
                Err(_) => {
                    eprintln!(
                        "Failed to receive response, server may be down. Reconnecting in 5s..."
                    );
                    break;
                }
            };

            if response.is_empty() {
                println!("Server closed the connection.");
                break;
            }
            println!("Received from server: {:?}", response);
            counter += 1;
            std::thread::sleep(Duration::from_secs(1));
        }

        match client.shutdown() {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to close connection: {}", e),
        };
    }
}
