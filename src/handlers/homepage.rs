use axum::response::IntoResponse;

use templates::HomepageTemplate;
use templates::response::HtmlResponse;

pub async fn homepage_handler() -> impl IntoResponse {
    HtmlResponse(HomepageTemplate {})
}
