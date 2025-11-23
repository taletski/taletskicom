use axum::routing::get;
use axum::{self, Router};

use crate::handlers::healthcheck_handler;
use crate::middleware::trace::get_configured_trace_layer;

pub fn create_router() -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck_handler))
        .layer(get_configured_trace_layer())
}
