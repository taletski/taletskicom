use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn healthcheck_handler() -> impl IntoResponse {
    let json_response = json!({
         "status": "ok",
         "message": "alive!"
    });

    Json(json_response)
}
