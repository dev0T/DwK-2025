use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::configuration::DatabaseSettings;

pub async fn connect_to_db(db_config: &DatabaseSettings) -> PgPool {
    log::info!(
        "[DB-CONNECT] Connecting to database {:?}:{:?} at {:?}.",
        db_config.name,
        db_config.port,
        db_config.host
    );
    let connection_string = db_config.connection_string();
    let db_pool = PgPool::connect(&connection_string.expose_secret())
        .await
        .expect("Connection to Postgres failed.");

    log::info!("[DB-CONNECT] Running migrations.");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect(format!("Failed to run migrations on {} DB.", db_config.name).as_str());
    log::info!("[DB-CONNECT] Done.");
    db_pool
}
