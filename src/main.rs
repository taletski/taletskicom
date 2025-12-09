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
use dotenvy::dotenv;

use std::io::Write;

fn debug_env_search() {
    let cwd = std::env::current_dir().expect("failed to get CWD");
    println!("CWD: {:?}", cwd);

    match std::fs::read_dir(&cwd) {
        Ok(_) => println!("can read dir"),
        Err(e) => {
            println!("can't read dir: {e}");
            return;
        },
    };

    let env_path = cwd.join(".env");
    match std::fs::read_to_string(env_path) {
        Ok(env) => println!("Read .env successfuly: {env}"),
        Err(err) => println!("Failed to read .env: {err}"),
    };

    for path in cwd.ancestors() {
        println!("-------------");
        println!("reading ancestor: {:?}", path);

        match std::fs::read_dir(path) {
            Ok(_) => println!("can read dir"),
            Err(e) => {
                println!("can't read dir: {e}");
                continue;
            },
        };

        let env_path = path.join(".env");
        if !env_path.exists() {
            println!("no .env here");
            continue;
        };

        match std::fs::read_to_string(env_path) {
            Ok(env) => println!("Read .env successfuly: {env}"),
            Err(err) => println!("Failed to read .env: {err}"),
        };
    }
    std::io::stdout().flush().expect("failed to flush stdout");
}

#[tokio::main]
async fn main() -> Result<()> {
    debug_env_search();

    dotenv().expect("Failed to init dotenv");

    let config = config::AppConfig::init();
    server::serve(&config).await?;

    Ok(())
}
