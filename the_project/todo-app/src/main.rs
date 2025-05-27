use actix_files::Files;
use actix_web::{middleware, App, HttpServer};
use anyhow::Result;
use dotenvy::dotenv;
use futures::StreamExt;
use log::{error, info};
use reqwest;
use std::env;
use tokio::fs::{create_dir_all, try_exists, File};
use tokio::io::copy;
use tokio_cron_scheduler::{Job, JobScheduler};

const DEFAULT_PORT: u16 = 8080;
const DESTINATION_PATH: &str = "./static/images";
const IMAGE_PATH: &str = "./static/images/image.jpg";
const IMAGE_API: &str = "https://picsum.photos/400/400";
const CRON_EXPRESSION: &str = "0 0/10 * * * *";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = env::var("PORT")
        .map(|port| match port.parse() {
            Ok(int) => int,
            Err(_) => {
                log::info!("Invalid PORT. Using default port {}.", DEFAULT_PORT);
                DEFAULT_PORT
            }
        })
        .unwrap_or(DEFAULT_PORT);

    create_dir_all(DESTINATION_PATH).await?;
    if !try_exists(IMAGE_PATH).await.unwrap() {
        match get_image().await {
            Ok(_) => log::info!("Image created"),
            Err(e) => log::error!("Unable to create image. {:?}", e),
        }
    }

    match create_schedule().await {
        Ok(_) => log::info!("Schedule created"),
        Err(_) => log::error!("Unable to create schedule"),
    }

    log::info!("Server started in port {}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(Files::new("/", "static").index_file("index.html"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn create_schedule() -> Result<()> {
    let schedule = JobScheduler::new().await?;

    schedule
        .add(Job::new_async(CRON_EXPRESSION, |_job_id, scheduler| {
            Box::pin(async move {
                match get_image().await {
                    Ok(_) => {
                        info!("Image updated");
                    }
                    Err(e) => error!("Unable to get image. {:?}", e),
                }
                time_till_update(scheduler).await;
            })
        })?)
        .await?;
    schedule.start().await?;

    time_till_update(schedule).await;

    Ok(())
}

async fn get_image() -> Result<()> {
    let mut file = File::create(IMAGE_PATH).await?;
    let mut stream = reqwest::get(IMAGE_API).await?.bytes_stream();
    while let Some(item) = stream.next().await {
        copy(&mut item?.as_ref(), &mut file).await?;
    }
    Ok(())
}

async fn time_till_update(mut sched: JobScheduler) -> () {
    let time_to_next = sched.time_till_next_job().await.unwrap().unwrap().as_secs();

    info!(
        "Next job will happen in {:?} minutes ({:?} seconds).",
        time_to_next / 60,
        time_to_next
    );
}
