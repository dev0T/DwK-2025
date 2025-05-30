use actix_web::{get, middleware, App, HttpResponse, HttpServer};
use anyhow::{anyhow, Result};
use dotenvy::dotenv;
use log::info;
use std::env;
use tokio::{fs::File, io::AsyncReadExt};

const FILE_PATH: &str = "./files/hash.txt";
const DEFAULT_PORT: u16 = 8080;
const DEFAULT_PINGPONG_URL: &str = "http://localhost:3006/";
const CM_FILE_PATH: &str = "./config/information.txt";

#[get("/")]
async fn index() -> HttpResponse {
    match format_data().await {
        Ok((string, pongs, file_contents, message)) => {
            let text = format!(
                "File content: {file_contents}\nenv variable: MESSAGE={message}\n{string}.\nPing / Pongs: {pongs}",
            );

            HttpResponse::Ok().body(text)
        }
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

async fn format_data() -> Result<(String, i32, String, String)> {
    let string = get_string().await?;
    let pongs = get_pongs().await?;
    let file_contents = get_file_from_cm().await?.trim_end().to_string();
    let message = get_message().await?;

    Ok((string, pongs, file_contents, message))
}

async fn get_string() -> Result<String> {
    match File::open(FILE_PATH).await {
        Ok(mut file) => {
            info!("Text file found.");
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;

            Ok(contents)
        }
        Err(e) => Err(anyhow!("Unable to open file: {:?}. {:?}", FILE_PATH, e)),
    }
}

async fn get_pongs() -> Result<i32> {
    let url: String = env::var("PINGPONG_URL").unwrap_or(DEFAULT_PINGPONG_URL.to_owned());

    let result = reqwest::get(url).await?.text().await?.parse()?;
    Ok(result)
}

async fn get_file_from_cm() -> Result<String> {
    match File::open(CM_FILE_PATH).await {
        Ok(mut file) => {
            info!("Text file found.");
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;

            Ok(contents)
        }
        Err(e) => Err(anyhow!("Unable to open file. {:?}", e)),
    }
}

async fn get_message() -> Result<String> {
    let message = env::var("MESSAGE");
    match message {
        Ok(msg) => Ok(msg),
        Err(e) => Err(anyhow!("Unable to read env variable MESSAGE. {:?}", e)),
    }
}
