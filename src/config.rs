use std::net::SocketAddr;

#[derive(Debug)]
pub struct AppConfig {
    pub server_addr: SocketAddr,
}

impl AppConfig {
    pub fn init() -> Self {
        let server_addr: SocketAddr = Self::get_var("SERVER_ADDR")
            .parse()
            .expect("SERVER_ADDR env var has to be a valid SocketAddr");
        Self { server_addr }
    }

    fn get_var(name: &str) -> String {
        std::env::var(name).expect(&format!("{name} should be set in .env"))
    }
}
