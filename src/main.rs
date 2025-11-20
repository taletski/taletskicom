mod serve;
mod route;
mod handlers;
mod config;

use anyhow::Result;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let config = config::AppConfig::init();
    serve::serve(&config).await?;

    Ok(())
}
