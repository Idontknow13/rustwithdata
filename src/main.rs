use anyhow::{Context, Result};
use axum::{routing::get, Router};

/// A basic "*hello*" GET endpoint.
async fn index() -> &'static str {
    "Hello, RustWithData!"
}

#[tokio::main]
async fn main() -> Result<()> {
    // enable tracing by default
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let app = Router::new().route("/", get(index));

    let addr = "[::]:8080";
    axum::Server::try_bind(&addr.parse()?)
        .context(format!("Failed to bind to address `{addr}`"))?
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
