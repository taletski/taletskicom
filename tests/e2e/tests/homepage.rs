use e2e::TestCtx;
use tokio;

#[e2e_ctx::inject]
#[tokio::test]
async fn homepage_can_be_loaded(ctx: TestCtx) {
    ctx.client
        .goto(&ctx.app_url("/"))
        .await
        .expect("Root homepage route is available");
    let body = ctx
        .client
        .source()
        .await
        .expect("Root homepage response body is readable");
    assert!(body.contains("Kirill Taletski"));
    assert!(body.contains("TypeScript"));
    assert!(body.contains("React"));
    assert!(body.contains("Product"));
}
