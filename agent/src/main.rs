mod network;
use network::Client;

fn main() -> std::io::Result<()> {
    let mut client = Client::new("127.0.0.1:1337")?;

    println!("Connected to server successfully!");

    loop {
        client.send(b"Hello, server!")?;
        let response = client.receive()?;
        if response.is_empty() {
            println!("Server closed the connection.");
            break;
        }
        println!("Received from server: {:?}", response);
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    client.shutdown()?;
    Ok(())
}
