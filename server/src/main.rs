use tokio::net::TcpListener;
use tracing::info;

mod misc;
mod network;

#[tokio::main]
async fn main() {
    misc::start_logger();
    let listener = TcpListener::bind("0.0.0.0:1337")
        .await
        .expect("Failed to bind to address");

    info!("Server started on 0.0.0.0:1337");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                info!("Accepted connection from {}", addr);
                tokio::spawn(async move {
                    let res = misc::handle_connection(stream).await;

                    if let Err(e) = res {
                        tracing::error!("Error in connection: {}", e);
                    }
                });
            }
            Err(e) => {
                tracing::error!("Failed to accept connection: {}", e);
            }
        }
    }
}
