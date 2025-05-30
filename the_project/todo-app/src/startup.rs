use crate::app_config::Settings;
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::{middleware, App, HttpServer};
use anyhow::Result;
use futures::StreamExt;
use log::{error, info};
use reqwest;
use std::net::TcpListener;
use tokio::fs::{create_dir_all, try_exists, File};
use tokio::io::copy;
use tokio_cron_scheduler::{Job, JobScheduler};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, anyhow::Error> {
        let listener = TcpListener::bind((config.host, config.port))?;
        let address = listener.local_addr().unwrap();
        log::info!("Starting HTTP server at {:?}", address);
        let server = start_server(listener).await?;

        create_dir_all(&config.image_dir).await?;

        let image_path = format!("{}/{}", &config.image_dir, config.image_path);

        if !try_exists(image_path).await.unwrap() {
            match get_image(&config.image_api).await {
                Ok(_) => log::info!("Image created"),
                Err(e) => log::error!("Unable to create image. {:?}", e),
            }
        }

        match create_schedule(&config.schedule_expression, config.image_api).await {
            Ok(_) => log::info!("Schedule created"),
            Err(_) => log::error!("Unable to create schedule"),
        }

        Ok(Self {
            port: address.port(),
            server,
        })
    }
    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn start_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(Files::new("/", "static").index_file("index.html"))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn create_schedule(expression: &str, url: String) -> Result<()> {
    let schedule = JobScheduler::new().await?;
    schedule
        .add(Job::new_async(expression, move |_job_id, scheduler| {
            let api_url = url.clone();
            Box::pin(async move {
                match get_image(&api_url).await {
                    Ok(_) => {
                        info!("Image updmovedated");
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

async fn get_image(api_url: &str) -> Result<()> {
    let mut file = File::create(api_url).await?;
    let mut stream = reqwest::get(api_url).await?.bytes_stream();
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
