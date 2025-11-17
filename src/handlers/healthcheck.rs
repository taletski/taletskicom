use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn healthcheck_handler() -> impl IntoResponse {
    let json_response = json!({
         "status": "success",
         "message": "alive!"
    });

    Json(json_response)
}
