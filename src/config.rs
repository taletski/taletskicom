use std::net::SocketAddr;
use strum::{self, VariantNames};

#[derive(Debug, PartialEq, Eq, strum::Display, strum::EnumString, strum::VariantNames)]
pub enum Env {
    Dev,
    Test,
    Prod,
}

#[derive(Debug)]
pub struct AppConfig {
    pub env: Env,
    pub server_addr: SocketAddr,
}

impl AppConfig {
    pub fn init() -> Self {
        let env: Env = Self::get_var("ENV").parse().unwrap_or_else(|_| {
            panic!(
                "Invalid ENV value in .env; valid options are {}",
                Env::VARIANTS.join(", ")
            )
        });
        let server_addr: SocketAddr = Self::get_var("SERVER_ADDR")
            .parse()
            .expect("SERVER_ADDR env var has to be a valid SocketAddr");
        Self { env, server_addr }
    }

    fn get_var(name: &str) -> String {
        std::env::var(name).unwrap_or_else(|_| panic!("{name} should be set in .env"))
    }
}
