use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, get, middleware, post, web};
use anyhow::Result;
use dotenvy::dotenv;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::sync::Mutex;

const DEFAULT_PORT: u16 = 8085;

struct AppState {
    todos: Mutex<Vec<Todo>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateTodo {
    title: String,
}

#[get("/v1/todos")]
async fn get_todos(data: web::Data<AppState>) -> HttpResponse {
    let todos = data.todos.lock().unwrap();

    HttpResponse::Ok().json(todos.clone())
}

#[post("/v1/todos")]
async fn create_todo(
    create_todo_request: web::Json<CreateTodo>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let mut todos = data.todos.lock().unwrap();

    let new_todo = Todo {
        title: create_todo_request.title.clone(),
    };

    todos.push(new_todo.clone());

    HttpResponse::Ok().json(new_todo)
}

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/v1")]
async fn index() -> HttpResponse {
    let response = json!({
        "name": "Todo Backend",
        "version": "1.0"
    });

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = env::var("PORT")
        .map(|port| match port.parse() {
            Ok(int) => int,
            Err(_) => {
                info!("Invalid PORT. Using default value.");
                DEFAULT_PORT
            }
        })
        .unwrap_or(DEFAULT_PORT);

    info!("Starting HTTP server at 0.0.0.0:{}", port);

    let state = web::Data::new(AppState {
        todos: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        // Should be changed to let cors = Cors::default().allowed_origin("https://myurl.org/")
        let cors = Cors::default().allowed_origin("http://todo-app-svc:2345");

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(state.clone())
            .service(
                web::scope("/api")
                    .service(get_todos)
                    .service(create_todo)
                    .service(index),
            )
            .service(health_check)
    })
    .bind(("0.0.0.0", port))
    .unwrap_or_else(|err| panic!("Unable to start the server in port {}: {:?}", port, err))
    .run()
    .await
}
