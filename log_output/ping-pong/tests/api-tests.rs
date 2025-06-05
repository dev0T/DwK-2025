use std::net::TcpListener;

use ping_pong::{configuration::DatabaseSettings, startup::start_server};
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let connection_pool = config_database().await;
    let port = listener.local_addr().unwrap().port();
    let server = start_server(listener, connection_pool.clone())
        .await
        .expect("Failed to start server.");

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: connection_pool,
    }
}

async fn config_database() -> PgPool {
    let db_config = DatabaseSettings {
        name: Uuid::new_v4().to_string(),
        user: "postgres".to_string(),
        password: "postgres".into(),
        port: 5432,
        host: "localhost".to_string(),
    };
    let connection_string = db_config.without_db();
    PgConnection::connect(&connection_string.expose_secret())
        .await
        .expect("Connection to Postgres failed.")
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_config.name).as_str())
        .await
        .expect("Failed to create test database");
    let db_pool = PgPool::connect(&db_config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to test DB");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect(format!("Failed to run migrations on {} DB.", db_config.name).as_str());
    db_pool
}

#[tokio::test]
async fn health_check() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn returns_counter_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn returns_updated_counter() {
    let app = spawn_app().await;

    let previous = sqlx::query!("SELECT pongs FROM pongs",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Unable to fetch from DB");

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/pingpong", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    let updated = sqlx::query!("SELECT pongs FROM pongs",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Unable to fetch from DB");

    assert!(response.status().is_success());
    assert_eq!(previous.pongs + 1, updated.pongs);
}
