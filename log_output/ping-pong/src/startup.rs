use crate::routes::{health_check, pingpong, pongs};
use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub async fn start_server(
    listener: TcpListener,
    connection_pool: PgPool,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(connection_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(db_pool.clone())
            .route("/pingpong", web::get().to(pingpong))
            .route("/health", web::get().to(health_check))
            .route("/", web::get().to(pongs))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
