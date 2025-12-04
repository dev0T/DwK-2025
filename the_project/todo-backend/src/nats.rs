use async_nats::Client;
use tracing::info;

use crate::app_config::NatsSettings;

pub async fn connect_to_nats(nats: &NatsSettings) -> Client {
    info!("[NATS-CONNECT] Connecting to NATS at {:?}.", nats.host);
    let connection_string = nats.connection_string();
    let client = async_nats::connect(connection_string)
        .await
        .expect("Connection to NATS failed.");
    info!("[NATS-CONNECT] Done.");
    client
}
