mod server;
mod route;
mod handlers;
mod config;
mod middleware;

use anyhow::Result;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let config = config::AppConfig::init();
    server::serve(&config).await?;

    Ok(())
}
