use e2e::TestCtx;
use tokio;

#[tokio::test]
async fn homepage() {
    let ctx = TestCtx::new().await;
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
