use fantoccini::{ClientBuilder, Locator};
use taletskicom;
use webdriver_install::Driver;

// #[cfg(not(test))]
// compile_error!("e2e crate is for tests only - do not depend on it");

static RUNTIME: std::sync::Once = std::sync::Once::new();

struct WebdriverHandle {
    process: std::process::Child,
}

pub enum Browser {
    Chrome,
}

impl Browser {
    pub fn get() -> Self {
        // hard-coded for now
        Browser::Chrome
    }

    pub fn launch(&self) -> WebdriverHandle {
        let config = taletskicom::config::AppConfig::init();
        let port = config.server_addr.port();
        let child = match self {
            Browser::Chrome => {
                webdriver_install::Driver::Chrome.install().expect("Chrome webdriver installed")
            }
        }
        // WebdriverHandle { process:  child }
    }
}

pub struct TestContext {
    pub tester: fantoccini::Client,
    pub base_url: String,
}

// impl TestContext {
//     pub async fn init() -> Self {
//         RUNTIME.call_once(|| {
//             chromedriver_autoinstaller
//         });
//     }
// }
