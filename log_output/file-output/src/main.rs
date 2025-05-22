use actix_web::{get, middleware, App, HttpResponse, HttpServer};
use anyhow::{anyhow, Result};
use dotenvy::dotenv;
use log::info;
use std::env;
use tokio::{fs::File, io::AsyncReadExt};
use uuid::Uuid;

const FILE_PATH: &str = "./files/file.txt";
const DEFAULT_PORT: u16 = 8080;

#[get("/")]
async fn index() -> HttpResponse {
    match format_data().await {
        Ok((timestamp, hash)) => HttpResponse::Ok().body(format!("{timestamp}:{hash}.")),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", err))
        }
    }
}

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("Version", "0.0.1"))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let port: u16 = env::var("PORT")
        .map(|port| match port.parse() {
            Ok(int) => int,
            Err(_e) => {
                println!("Invalid PORT. Using default value.");
                DEFAULT_PORT
            }
        })
        .unwrap_or(DEFAULT_PORT);

    info!("Starting HTTP server at 0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(health_check)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn format_data() -> Result<(String, Uuid)> {
    let timestamp = get_timestamp().await?;
    let hash: Uuid = get_hash().await?;

    Ok((timestamp, hash))
}

async fn get_timestamp() -> Result<String> {
    match File::open(FILE_PATH).await {
        Ok(mut file) => {
            info!("Text file found.");
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;

            Ok(contents)
        }
        Err(e) => Err(anyhow!("Unable to open file. {:?}", e)),
    }
}

async fn get_hash() -> Result<Uuid> {
    let hash = Uuid::new_v4();
    Ok(hash)
}
