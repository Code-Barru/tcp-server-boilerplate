use std::sync::Arc;
use tokio::net::TcpStream;
use tracing::info;

use shared::error::NetworkError;
use crate::network::{perform_handshake, MultiplexManager};

pub async fn handle_connection(stream: TcpStream) -> Result<(), NetworkError> {
    let (mut read_half, mut write_half) = stream.into_split();
    let ip = read_half.peer_addr()?;
    let shared_secret = perform_handshake(&mut read_half, &mut write_half).await?;

    let manager = Arc::new(MultiplexManager::new(read_half, write_half, shared_secret));

    manager.start();

    info!("Connection established with {}, opening streams...", ip);

    let mut handles = vec![];
    for i in 0..3 {
        let manager_clone = manager.clone();
        let handle = tokio::spawn(async move {
            match manager_clone.open_stream().await {
                Ok(mut stream) => {
                    info!("Stream {} opened (id={})", i, stream.id());

                    for j in 0..5 {
                        let message = format!("Message {} from server stream {}", j, i);
                        if let Err(e) = stream.send_bytes(message.as_bytes()).await {
                            tracing::error!("Stream {}: failed to send: {}", i, e);
                            break;
                        }
                        info!("Stream {}: sent message {}", i, j);

                        match stream.receive().await {
                            Ok(data) => {
                                info!(
                                    "Stream {}: received echo: {:?}",
                                    i,
                                    String::from_utf8_lossy(&data)
                                );
                            }
                            Err(e) => {
                                tracing::error!("Stream {}: failed to receive: {}", i, e);
                                break;
                            }
                        }

                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    }

                    info!("Stream {} finished", i);
                }
                Err(e) => {
                    tracing::error!("Failed to open stream {}: {}", i, e);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    info!("All streams finished for {}", ip);
    Ok(())
}
