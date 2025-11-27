use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use taletskicom::config::AppConfig;

const IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const PORT: u16 = 8080;

pub struct AppConfigMock {}

impl AppConfigMock {
    pub fn init() -> &'static AppConfig {
        static MOCK_APP_CONFIG: AppConfig = AppConfig {
            server_addr: SocketAddr::new(IP, PORT),
        };
        &MOCK_APP_CONFIG
    }
}
