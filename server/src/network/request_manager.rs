use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use shared::packets::Frame;

use crate::network::NetworkError;

pub struct FrameIterator {
    rx: mpsc::Receiver<Frame>,
}

impl FrameIterator {
    pub async fn next_frame(&mut self) -> Option<Frame> {
        self.rx.recv().await
    }
}

#[derive(Clone)]
pub struct RequestManager {
    next_id: Arc<AtomicU64>,
    pending: Arc<Mutex<HashMap<u64, mpsc::Sender<Frame>>>>,
}

impl RequestManager {
    pub fn new() -> Self {
        RequestManager {
            next_id: Arc::new(AtomicU64::new(1)),
            pending: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn next_request_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    pub async fn register_request(&self, request_id: u64) -> Result<FrameIterator, NetworkError> {
        let (tx, rx) = mpsc::channel(100);
        let mut pending = self.pending.lock().await;
        pending.insert(request_id, tx);
        Ok(FrameIterator { rx })
    }

    pub async fn route_response(&self, frame: Frame) -> Result<bool, NetworkError> {
        let mut pending = self.pending.lock().await;
        let request_id = frame.request_id;

        if let Some(tx) = pending.get(&request_id) {
            let is_last = frame.is_last;
            let _ = tx.send(frame).await;

            if is_last {
                pending.remove(&request_id);
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[allow(dead_code)]
    pub async fn cancel_request(&self, request_id: u64) -> Result<bool, NetworkError> {
        let mut pending = self.pending.lock().await;
        Ok(pending.remove(&request_id).is_some())
    }

    #[allow(dead_code)]
    pub async fn pending_count(&self) -> Result<usize, NetworkError> {
        let pending = self.pending.lock().await;
        Ok(pending.len())
    }

    #[allow(dead_code)]
    pub async fn cancel_all(&self) -> Result<(), NetworkError> {
        let mut pending = self.pending.lock().await;
        pending.clear();
        Ok(())
    }
}

impl Default for RequestManager {
    fn default() -> Self {
        Self::new()
    }
}
