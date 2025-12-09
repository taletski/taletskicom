#![deny(
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::dbg_macro,
    clippy::unimplemented
)]

mod config;
mod handlers;
mod middleware;
mod response;
mod route;
mod server;
mod templates;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::AppConfig::init();

    server::serve(&config).await?;

    Ok(())
}
