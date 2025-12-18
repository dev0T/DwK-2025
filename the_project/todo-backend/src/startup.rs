use crate::app_config::Settings;
use crate::db::connect_to_db;
use crate::nats::connect_to_nats;
use crate::routes;
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use async_nats::Client;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::info;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, anyhow::Error> {
        let listener = TcpListener::bind((config.host.clone(), config.port))?;
        let db_connection = connect_to_db(&config.database).await;
        let nats_client = connect_to_nats(&config.nats).await;
        let address = listener.local_addr().unwrap();
        info!("Starting HTTP server at {:?}", address);
        let sub_path = config.get_sub_path();
        let server =
            start_server(listener, db_connection, nats_client, sub_path.to_string()).await?;

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

async fn start_server(
    listener: TcpListener,
    connection_pool: PgPool,
    nats: Client,
    sub_path: String,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(connection_pool);
    let nats_client = web::Data::new(nats);

    let server = HttpServer::new(move || {
        let cors = if sub_path == "development/" {
            Cors::permissive()
        } else {
            //
            // let cors = Cors::default().allowed_origin_fn(|origin, _req_head| {
            //     origin.as_bytes().starts_with(b"http://localhost")
            // });
            Cors::default()
        };

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .app_data(db_pool.clone())
            .app_data(nats_client.clone())
            .configure(routes::health::service)
            .service(
                web::scope(format!("/{sub_path}api").as_str()).configure(routes::todos::service),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
