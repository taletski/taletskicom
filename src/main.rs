#![deny(
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::dbg_macro,
    clippy::unimplemented
)]

mod config;
mod handlers;
mod middleware;
mod route;
mod server;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::AppConfig::init();

    server::serve(&config).await?;

    Ok(())
}
