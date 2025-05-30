use crate::app_config::Settings;
use crate::models::todo::Todo;
use crate::routes;
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, middleware, web};
use std::net::TcpListener;
use std::sync::Mutex;

pub struct AppState {
    pub todos: Mutex<Vec<Todo>>,
}

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
    let state = web::Data::new(AppState {
        todos: Mutex::new(Vec::new()),
    });

    let server = HttpServer::new(move || {
        let cors = Cors::default().allowed_origin_fn(|origin, _req_head| {
            origin.as_bytes().starts_with(b"http://localhost")
        });
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(state.clone())
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
