use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use log::info;
use std::env;

const DEFAULT_PORT: u16 = 8080;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = env::var("PORT")
        .map(|port| match port.parse() {
            Ok(int) => int,
            Err(_) => {
                info!("Invalid PORT. Using default port {}.", DEFAULT_PORT);
                DEFAULT_PORT
            }
        })
        .unwrap_or(DEFAULT_PORT);

    info!("Server started in port {}", port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
