use dotenvy::dotenv;
use log;
use ping_pong::configuration::get_app_config;
use ping_pong::db::connect_to_db;
use ping_pong::startup::start_server;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = get_app_config().expect("Unable to get configuration.");
    let connection = connect_to_db(&config.database).await;

    let port = config.port;

    let listener = TcpListener::bind(("0.0.0.0", port))?;

    log::info!("Starting HTTP server at {listener:?}:{port:?}");
    start_server(listener, connection).await?.await
}
