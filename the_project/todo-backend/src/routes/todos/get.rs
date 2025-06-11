use actix_web::{HttpResponse, web};
use log::error;
use sqlx::PgPool;

use crate::models::todo::Todo;

pub async fn all(db_pool: web::Data<PgPool>) -> HttpResponse {
    // get data from DB

    let query_result = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(db_pool.get_ref())
        .await;

    match query_result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => {
            error!("Unable to execute query: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
