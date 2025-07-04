use anyhow::Result;
use dotenvy::dotenv;
use todo_backend::app_config::get_app_config;
use todo_backend::startup::Application;
use todo_backend::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let subscriber = get_subscriber("todo-app".into(), "info".into());
    init_subscriber(subscriber);

    let config = get_app_config().expect("Unable to get configuration.");
    let application = Application::build(config).await?;
    let app_task = tokio::spawn(application.run());

    tokio::select! {
        _ = app_task => {},
    }

    Ok(())
}
