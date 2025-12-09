use anyhow::Result;
use axum::routing::get;
use axum::{self, Router};
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use crate::config::{AppConfig, Env};
use crate::handlers::{healthcheck_handler, homepage_handler};
use crate::middleware::trace::get_configured_trace_layer;

pub fn create_router(config: &AppConfig) -> Result<Router> {
    let mut router = Router::new()
        .route("/", get(homepage_handler))
        .route("/healthcheck", get(healthcheck_handler))
        .layer(get_configured_trace_layer())
        .nest_service(
            "/assets",
            ServeDir::new(format!(
                "{}/assets",
                std::env::current_dir()
                    .unwrap_or_else(|_| panic!(
                        "Failed to current dir. The assets dir can not be served."
                    ))
                    .display()
            )),
        );

    if config.env == Env::Dev {
        router = router.layer(LiveReloadLayer::new());
    }

    router = router.layer(CompressionLayer::new());

    Ok(router)
}
