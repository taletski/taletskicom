use axum::routing::get;
use axum::{self, Router};

use crate::handlers::healthcheck_handler;

pub fn create_router() -> Router {
    Router::new().route("/healthcheck", get(healthcheck_handler))
}
