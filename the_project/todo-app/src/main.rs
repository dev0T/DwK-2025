use anyhow::Result;
use dotenvy::dotenv;
use todo_app::{app_config::get_app_config, startup::Application};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = get_app_config().expect("Unable to get configuration.");

    let application = Application::build(config).await?;

    let app_task = tokio::spawn(application.run());

    tokio::select! {
        _ = app_task => {},
    }

    Ok(())
}
