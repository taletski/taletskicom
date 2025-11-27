use anyhow::Result;

mod config_mock;
use config_mock::AppConfigMock;

mod browser;
use browser::Browser;

mod webdriver_server;
use webdriver_server::WebDriverServer;

use taletskicom;

// #[cfg(not(test))]
// compile_error!("E2E crate is for tests only - do not depend on it");

pub struct TestCtx {
    pub client: thirtyfour::WebDriver,
    pub base_url: String,
    _driver: WebDriverServer,
    _server: tokio::task::JoinHandle<Result<()>>,
}

impl TestCtx {
    pub async fn new() -> Self {
        let mock_app_config = AppConfigMock::init();
        let _server = tokio::task::spawn(taletskicom::server::serve(mock_app_config));

        let browser = Browser::current();
        let web_driver_server = WebDriverServer::launch(&browser).await;
        let test_client = browser.connect_test_client(web_driver_server.port).await;

        TestCtx {
            client: test_client,
            base_url: format!("http://{}", mock_app_config.server_addr.to_string()),
            _driver: web_driver_server,
            _server,
        }
    }

    pub fn app_url(&self, rel_path: &str) -> String {
        format!("{}{}", &self.base_url, rel_path)
    }
}

impl Drop for TestCtx {
    fn drop(&mut self) {
        // Not doing self.client.quit() because it allows checking browser state on failed
        // tests in the headed mode.
    }
}
