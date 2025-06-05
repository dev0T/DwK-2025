use actix_web::{web, HttpResponse};
use log::error;
use sqlx::PgPool;

pub async fn pingpong(db_pool: web::Data<PgPool>) -> HttpResponse {
    let query_result =
        sqlx::query!("UPDATE pongs SET pongs = pongs + 1 WHERE id = 1 RETURNING pongs")
            .fetch_one(db_pool.get_ref())
            .await;
    match query_result {
        Ok(result) => {
            let counter: i32 = result.pongs;
            HttpResponse::Ok().json(counter)
        }
        Err(err) => {
            error!("Unable to execute query: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn pongs(db_pool: web::Data<PgPool>) -> HttpResponse {
    let query_result = sqlx::query!("SELECT * FROM pongs")
        .fetch_one(db_pool.get_ref())
        .await;
    match query_result {
        Ok(result) => HttpResponse::Ok().json(result.pongs),
        Err(err) => {
            error!("Unable to execute query: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
