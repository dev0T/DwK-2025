use actix_web::{HttpResponse, web};
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::todo::{Todo, TodoTitle};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodo {
    title: String,
}

pub async fn create(
    db_pool: web::Data<PgPool>,
    create_todo_request: web::Json<CreateTodo>,
) -> HttpResponse {
    let title = TodoTitle::parse(create_todo_request.title.clone()).unwrap();
    let new_todo: Todo = Todo::new(Uuid::new_v4(), title);

    let query_result = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (id, title) VALUES ($1, $2)
        "#,
        new_todo.id,
        new_todo.title.as_ref()
    )
    .execute(db_pool.get_ref())
    .await;

    match query_result {
        Ok(_result) => HttpResponse::Ok().json(new_todo),
        Err(err) => {
            error!("Unable to execute query: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
