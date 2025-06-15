use std::io;
use tokio::net::TcpStream;
use tracing::info;

use crate::network::{Connection, perform_handshake};

pub async fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let (mut read_half, mut write_half) = stream.into_split();
    let ip = read_half.peer_addr()?;
    let shared_secret = perform_handshake(&mut read_half, &mut write_half).await?;
    let mut connection = Connection::new(read_half, write_half, shared_secret)?;

    loop {
        let response = match connection.receive().await {
            Ok(data) => data,
            Err(e) => {
                tracing::error!("Failed to receive data: {}", e);
                break;
            }
        };

        if response.is_empty() {
            info!("Connection closed by client: {}", ip);
            break;
        }

        info!("Received data from {}: {:?}", ip, response);
        if let Err(e) = connection.send(&response).await {
            tracing::error!("Failed to send data: {}", e);
            break;
        }
    }

    Ok(())
}
