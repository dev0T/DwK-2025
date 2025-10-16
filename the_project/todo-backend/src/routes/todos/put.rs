use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use tracing::{Instrument, error, info_span};
use uuid::Uuid;

use crate::models::todo::Todo;

pub async fn done(db_pool: web::Data<PgPool>, id: web::Path<Uuid>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    let todo_id = id.into_inner();

    let request_span = info_span!("Updating todo status.", %request_id);

    let _request_span_guard = request_span.enter();

    let query_span = info_span!("Updating todo in the database");

    let query_result = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos SET done = NOT done WHERE id = $1 RETURNING *
        "#,
        todo_id
    )
    .fetch_one(db_pool.get_ref())
    .instrument(query_span)
    .await;

    match query_result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => {
            error!("Unable to execute query: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
