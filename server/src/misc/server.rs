use std::time::Duration;

use shared::packets::{FileChunk, Packet, PacketType, PingRequest};
use tokio::net::TcpStream;
use tracing::info;

use crate::network::{Connection, NetworkError, perform_handshake};

pub async fn handle_connection(stream: TcpStream) -> Result<(), NetworkError> {
    let (mut read_half, mut write_half) = stream.into_split();
    let ip = read_half.peer_addr()?;
    info!("Starting handshake with {}", ip);

    let shared_secret = perform_handshake(&mut read_half, &mut write_half).await?;
    let (connection, handle) = Connection::new(read_half, write_half, shared_secret);

    info!(
        "Handshake complete with {}, starting connection handler",
        ip
    );

    handle.spawn_response_router();

    let handle_clone = handle.clone();
    tokio::spawn(async move {
        for i in 0..5 {
            tokio::time::sleep(Duration::from_secs(5)).await;

            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            let ping_request = PingRequest::new(timestamp);
            let payload = match ping_request.serialize() {
                Ok(data) => data[1..].to_vec(),
                Err(e) => {
                    tracing::error!("Failed to serialize ping request: {}", e);
                    break;
                }
            };

            match handle_clone
                .send_request_and_wait(PacketType::PingRequest, payload, Duration::from_secs(5))
                .await
            {
                Ok(_response) => {
                    info!("Received pong #{} from {}", i + 1, ip);
                }
                Err(e) => {
                    tracing::error!("Failed to send/receive ping #{}: {}", i + 1, e);
                    break;
                }
            }
        }
        info!("Ping task finished for {}", ip);
    });

    tokio::spawn(async move {
        test_multiframe_request(handle).await;
    });

    connection.run().await
}

async fn test_multiframe_request(handle: crate::network::ConnectionHandle) {
    info!("=== Starting multi-frame test ===");
    info!("This test simulates receiving a file in 5 chunks from the agent");

    let mut request = match handle.send_request(PacketType::FileChunkTest, vec![]).await {
        Ok(req) => req,
        Err(e) => {
            tracing::error!("Failed to initiate multi-frame request: {}", e);
            return;
        }
    };

    info!("Request sent, waiting for chunks from agent...");
    info!("(Agent should respond with 5 FileChunk frames with is_last=true on the last one)");

    let mut received_chunks = 0;
    let mut total_bytes = 0;

    while let Some(frame) = request.next_frame().await {
        received_chunks += 1;

        match FileChunk::deserialize(&frame.payload) {
            Ok(chunk) => {
                total_bytes += chunk.data.len();
                info!(
                    "✓ Chunk #{}: {} bytes (is_last: {})",
                    chunk.chunk_number,
                    chunk.data.len(),
                    frame.is_last
                );
            }
            Err(e) => {
                tracing::error!("✗ Failed to deserialize chunk: {}", e);
            }
        }

        if frame.is_last {
            info!("=== Multi-frame test completed ===");
            info!("Total chunks received: {}", received_chunks);
            info!("Total bytes received: {}", total_bytes);
            break;
        }
    }
}
