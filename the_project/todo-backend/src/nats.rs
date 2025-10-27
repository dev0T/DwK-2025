use async_nats::Client;
use tracing::info;

pub async fn connect_to_nats(url: &str) -> Client {
    info!("[NATS-CONNECT] Connecting to NATS at {:?}.", url);
    let client = async_nats::connect(url)
        .await
        .expect("Connection to NATS failed.");
    info!("[NATS-CONNECT] Done.");
    client
}
