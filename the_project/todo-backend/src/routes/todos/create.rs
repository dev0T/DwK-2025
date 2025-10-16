use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{Instrument, error, info_span};
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
    let request_id = Uuid::new_v4();
    let title = create_todo_request.title.clone();

    let request_span = info_span!("Adding a new todo.", %request_id, todo_title = %title);

    let _request_span_guard = request_span.enter();

    let title = TodoTitle::parse(title);

    match title {
        Ok(title) => {
            let new_todo: Todo = Todo::new(Uuid::new_v4(), title, false);
            let query_span = info_span!("Saving new todo in the database");
            let query_result = sqlx::query_as!(
                Todo,
                r#"
                INSERT INTO todos (id, title, done) VALUES ($1, $2, $3)
                "#,
                new_todo.id,
                new_todo.title.as_ref(),
                new_todo.done
            )
            .execute(db_pool.get_ref())
            .instrument(query_span)
            .await;

            match query_result {
                Ok(_result) => HttpResponse::Ok().json(new_todo),
                Err(err) => {
                    error!("Unable to execute query: {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(err) => {
            error!("Error when creating todo: {}", err);
            HttpResponse::BadRequest().finish()
        }
    }
}
