use anyhow::Result;
use tracing::info;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::AppConfig;
use crate::route::create_router;

pub async fn serve(config: &AppConfig) -> Result<()> {
    #[allow(clippy::print_stdout)]
    {
        println!("⏳ Starting server on {0}", config.server_addr);
    }

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "taletskicom=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let router = create_router()?;
    info!("✅ Router ready");

    let listener = tokio::net::TcpListener::bind(config.server_addr).await?;
    info!("✅ TCP listener bound to {0}", config.server_addr);

    let server_future = axum::serve(listener, router.into_make_service());
    info!("✅ Server is listening on {0}", config.server_addr);

    server_future.await?;

    Ok(())
}
