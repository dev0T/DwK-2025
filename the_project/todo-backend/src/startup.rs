use crate::app_config::Settings;
use crate::db::connect_to_db;
use crate::routes;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::info;
use tracing_actix_web::TracingLogger;

//use actix_cors::Cors;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, anyhow::Error> {
        let listener = TcpListener::bind((config.host, config.port))?;
        let connection = connect_to_db(&config.database).await;
        let address = listener.local_addr().unwrap();
        info!("Starting HTTP server at {:?}", address);
        let server = start_server(listener, connection).await?;

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
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(connection_pool);

    let server = HttpServer::new(move || {
        // Only for development
        // let cors = Cors::permissive();
        //
        // let cors = Cors::default().allowed_origin_fn(|origin, _req_head| {
        //     origin.as_bytes().starts_with(b"http://localhost")
        // });
        App::new()
            .wrap(TracingLogger::default())
            //.wrap(cors)
            .app_data(db_pool.clone())
            .configure(routes::health::service)
            .service(
                web::scope("/api")
                    .configure(routes::todos::service)
                    .configure(routes::health::service),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
