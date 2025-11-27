use crate::browser::Browser;

use std::process::Command;

use portpicker;
use reqwest;
use tokio::time::Duration;

pub struct WebDriverServer {
    pub port: u16,
    process: std::process::Child,
}

impl WebDriverServer {
    const START_WAIT_TIMEOUT: Duration = Duration::from_secs(10);

    pub async fn launch(browser: &Browser) -> Self {
        let port = portpicker::pick_unused_port()
            .expect(&format!("No free port found for the {} webdriver", browser));
        let process = Command::new(browser.webdriver_bin_name())
            .arg(browser.webdriver_bin_args(port))
            .spawn()
            .expect("Failed to start chromedriver");
        // Prevents WebDriver process leaks.
        // Manager is dropped and WebDriver process is killed in case of a downstream panic.
        let manager = Self { process, port };
        manager.wait_for_webdriver_ready(browser, port).await;
        manager
    }

    pub fn get_webdriver_base_url(port: u16) -> String {
        format!("http://localhost:{port}")
    }

    pub fn build_webdriver_url(port: u16, path: &str) -> String {
        let mut result = WebDriverServer::get_webdriver_base_url(port);
        result.push_str(path);
        result
    }

    async fn wait_for_webdriver_ready(&self, browser: &Browser, port: u16) {
        let endpoint = "/status";
        let addr = WebDriverServer::build_webdriver_url(port, endpoint);
        tokio::time::timeout(WebDriverServer::START_WAIT_TIMEOUT, async {
            loop {
                println!("connecting to {}", &addr);
                if reqwest::get(&addr)
                    .await
                    .is_ok_and(|res| res.status().is_success())
                {
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        })
        .await
        .expect(&format!(
            "{} webdriver endpoint {} did not respond in {}s",
            browser,
            endpoint,
            WebDriverServer::START_WAIT_TIMEOUT.as_secs()
        ));
    }
}

impl Drop for WebDriverServer {
    fn drop(&mut self) {
        match self.process.kill() {
            Err(error) => {
                eprintln!(
                    "Failed to kill webdriver process with id {}. Error: {error}",
                    self.process.id()
                )
            },
            _ => {},
        }
    }
}
