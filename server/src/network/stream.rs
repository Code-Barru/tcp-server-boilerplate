use shared::{error::NetworkError, multiplexing::StreamId, packets::Packet};
use std::sync::Arc;
use tokio::sync::mpsc;

use super::multiplex::MultiplexManager;

pub struct Stream {
    pub id: StreamId,
    pub manager: Arc<MultiplexManager>,
    pub rx: mpsc::Receiver<Vec<u8>>,
}

impl Stream {
    pub fn new(
        id: StreamId,
        manager: Arc<MultiplexManager>,
        rx: mpsc::Receiver<Vec<u8>>,
    ) -> Self {
        Self { id, manager, rx }
    }

    pub fn id(&self) -> StreamId {
        self.id
    }

    pub async fn send<P: Packet>(&self, packet: P) -> Result<(), NetworkError> {
        let data = packet.serialize()?;
        self.send_bytes(&data).await
    }

    pub async fn send_bytes(&self, data: &[u8]) -> Result<(), NetworkError> {
        self.manager.send_on_stream(self.id, data.to_vec()).await
    }

    pub async fn receive(&mut self) -> Result<Vec<u8>, NetworkError> {
        self.rx
            .recv()
            .await
            .ok_or(NetworkError::ChannelReceiveError)
    }

    pub async fn close(self) -> Result<(), NetworkError> {
        // The stream will be automatically closed when dropped
        // The MultiplexManager will detect this and clean up
        Ok(())
    }
}
