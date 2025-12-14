use e2e::TestCtx;
use tokio;

#[e2e_ctx::inject]
#[tokio::test]
async fn healthcheck_responds_ok() {
    ctx.client
        .goto(&ctx.app_url("/healthcheck"))
        .await
        .expect("/healthcheck route is available");
    let body = ctx
        .client
        .source()
        .await
        .expect("/healthcheck response body is readable");
    assert!(body.contains(r#"status":"ok"#));
}
