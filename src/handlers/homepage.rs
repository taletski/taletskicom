use axum::response::IntoResponse;

use crate::response::HtmlResponse;
use crate::templates::HomepageTemplate;

pub async fn homepage_handler() -> impl IntoResponse {
    HtmlResponse(HomepageTemplate {})
}
