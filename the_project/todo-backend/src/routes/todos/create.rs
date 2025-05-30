use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::todo::{Todo, TodoTitle},
    startup::AppState,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodo {
    title: String,
}

pub async fn create(
    data: web::Data<AppState>,
    create_todo_request: web::Json<CreateTodo>,
) -> HttpResponse {
    let mut todos = data.todos.lock().unwrap();

    let title = TodoTitle::parse(create_todo_request.title.clone()).unwrap();

    let new_todo: Todo = Todo::new(Uuid::new_v4(), title);
    todos.push(new_todo.clone());

    HttpResponse::Ok().json(new_todo)
}
