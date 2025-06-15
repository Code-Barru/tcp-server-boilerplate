mod network;
use network::Client;

fn main() {
    loop {
        let mut client = loop {
            match Client::new("127.0.0.1:1337") {
                Ok(client) => break client,
                Err(_) => {
                    eprintln!("Failed to connect to server, retrying in 5 seconds..");
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            }
        };
        println!("Connected to server successfully!");

        loop {
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
            std::thread::sleep(std::time::Duration::from_secs(5));
        }

        match client.shutdown() {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to close connection: {}", e),
        };
    }
}
