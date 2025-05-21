use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use dotenv::dotenv;
use log::info;
use std::env;
use uuid::Uuid;

const DEFAULT_PORT: u16 = 8080;

struct AppState {
    id: Uuid,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let timestamp: chrono::DateTime<Utc> = Utc::now();
    let id: Uuid = data.id;

    HttpResponse::Ok().body(format!("{timestamp:?}: {id}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

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
            .app_data(web::Data::new(AppState {
                id: Uuid::from(Uuid::new_v4()),
            }))
            .service(index)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
