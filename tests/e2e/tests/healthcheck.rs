use tokio;
use e2e::TestCtx;

#[tokio::test]
async fn healthcheck() {
    let ctx = TestCtx::new().await;
    ctx.client.goto(&ctx.app_url("/healthcheck")).await.expect("/healthcheck route is available");
    let body = ctx.client.source().await.expect("/healthcheck response body is readable");
    assert!(body.contains(r#"status":"ok"#));
}
