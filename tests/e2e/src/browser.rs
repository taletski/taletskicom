use thirtyfour::{self, ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};
use crate::webdriver_server::WebDriverServer;
use strum;

#[derive(strum::Display)]
pub enum Browser {
    Chrome,
}

impl Browser {
    pub fn current() -> Self {
        // Hard-coded for now
        // Later on should be controlled by ENV
        Browser::Chrome
    }

    pub fn webdriver_bin_name (&self) -> &'static str {
        match self {
            Browser::Chrome => "chromedriver",
        } 
    }

    pub fn webdriver_bin_args(&self, port: u16) -> String {
        match self {
            Browser::Chrome => format!("--port={port}"),
        } 
    }

    pub async fn connect_test_client(&self, port: u16) -> thirtyfour::WebDriver {
        let mut capabilities = match self {
            Browser::Chrome => {
                let mut caps = DesiredCapabilities::chrome();
                // `/dev/shm` used by Chrome as a small shared memory space. It is often too
                // small in container environments, leading to browser crashes
                caps.add_arg("--disable-dev-shm-usage")
                    .expect("Failed to add the flag for disabling Chrome's /dev/shm usage");
                // Container environments often lack the kernel security features required
                // for sandboxing, causing Chrome to crash on launch
                caps.add_arg("--no-sandbox")
                    .expect("Failed to add the flag disabling Chrome sandbox");
                caps
            }
        };
        // Disable TLS errors for local testing
        capabilities
            .set_ignore_certificate_errors()
            .expect("Failed to ignore TLS errors");
        capabilities
            .set_headless()
            .expect("Failed to set test browser in headless mode");
        WebDriver::new(WebDriverServer::get_webdriver_base_url(port), capabilities)
            .await
            .expect("Failed to initialize WebDriver")
    }
}
