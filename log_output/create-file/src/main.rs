use actix_web::{middleware, App, HttpServer};
use anyhow::Result;
use chrono::Utc;
use dotenvy::dotenv;
use log::{error, info};
use std::io::prelude::*;
use std::{env, fs::OpenOptions};
use tokio::fs::create_dir_all;
use tokio_cron_scheduler::{Job, JobScheduler};

const FILE_PATH: &str = "./files/file.txt";
const CRON_EXPRESSION: &str = "*/5 * * * * *";
const DEFAULT_PORT: u16 = 8083;

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

    log::info!("Starting HTTP server at 0.0.0.0:{}", port);

    create_dir_all("./files").await?;

    match create_schedule().await {
        Ok(_) => info!("Schedule created."),
        Err(_) => error!("Unable to create schedule."),
    }

    HttpServer::new(move || App::new().wrap(middleware::Logger::default()))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}

async fn create_schedule() -> Result<()> {
    let schedule = JobScheduler::new().await?;

    schedule
        .add(Job::new_async(CRON_EXPRESSION, |_job_id, _scheduler| {
            Box::pin(async move {
                match generate_timestamp().await {
                    Ok(_) => info!("File updated"),
                    Err(e) => error!("Unable to update file. {:?}", e),
                }
            })
        })?)
        .await?;
    schedule.start().await?;
    Ok(())
}

async fn generate_timestamp() -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .unwrap();
    let timestamp: chrono::DateTime<Utc> = Utc::now();
    write!(file, "{timestamp:?}").expect("Cannot write to file");
    Ok(())
}
