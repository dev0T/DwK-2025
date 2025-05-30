use actix_web::{HttpResponse, web};

use crate::startup::AppState;

pub async fn all(data: web::Data<AppState>) -> HttpResponse {
    let todos = data.todos.lock().unwrap();

    HttpResponse::Ok().json(todos.clone())
}
