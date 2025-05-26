use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use log::info;
use std::{env, fs::OpenOptions, io::prelude::*, ops::Add, sync::Mutex};

const DEFAULT_PORT: u16 = 8080;
const FILE_PATH: &str = "./files/counter.txt";

struct AppState {
    counter: Mutex<u32>,
}

#[get("/pingpong")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut file = OpenOptions::new()
        .read(true)
        .append(false)
        .write(true)
        .create(true)
        .open(FILE_PATH)
        .unwrap();

    let mut counter = data.counter.lock().unwrap();

    *counter = counter.add(1);
    write!(&mut file, "{counter:?}").expect("Cannot write to file");
    HttpResponse::Ok().body(format!("Pong {counter}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut file = OpenOptions::new()
        .read(true)
        .append(false)
        .write(true)
        .create(true)
        .open(FILE_PATH)
        .unwrap();

    let mut initial_value: u32 = 0;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    match contents.parse() {
        Ok(value) => {
            initial_value = value;
        }
        Err(_) => {
            info!("Invalid file contents. Replacing with 0.");
            write!(&mut file, "0").expect("Cannot write to file");
        }
    }

    let counter = web::Data::new(AppState {
        counter: Mutex::new(initial_value),
    });

    info!("{} pongs found", initial_value);

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
