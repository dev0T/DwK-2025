use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use log::info;
use std::{env, sync::Mutex};

const DEFAULT_PORT: u16 = 8080;

struct AppState {
    counter: Mutex<i32>,
}

#[get("/pingpong")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Pong {counter}"))
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

    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(counter.clone())
            .service(index)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
