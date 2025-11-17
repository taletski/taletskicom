mod config;
mod handlers;
mod route;
mod serve;

use anyhow::Result;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let config = config::AppConfig::init();
    serve::serve(&config).await?;

    Ok(())
}
