use actix_web::{middleware, App, HttpServer};
use anyhow::Result;
use chrono::Utc;
use dotenvy::dotenv;
use log::{error, info};
use std::env;
use std::io::Write;
use tokio::{
    fs::{create_dir_all, OpenOptions},
    io::AsyncWriteExt,
};
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

const FILE_PATH: &str = "./files/hash.txt";
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
    let hash = Uuid::new_v4();

    schedule
        .add(Job::new_async(
            CRON_EXPRESSION,
            move |_job_id, _schedulerenv| {
                Box::pin(async move {
                    match update_file(hash).await {
                        Ok(_) => info!("File updated"),
                        Err(e) => error!("Unable to update file. {:?}", e),
                    }
                })
            },
        )?)
        .await?;
    schedule.start().await?;
    Ok(())
}

async fn update_file(hash: Uuid) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .await?;
    let timestamp: chrono::DateTime<Utc> = Utc::now();
    let mut buf: Vec<u8> = Vec::<u8>::new();
    write!(buf, "{timestamp:?}: {hash:?}")?;
    file.write(&buf).await?;
    Ok(())
}
