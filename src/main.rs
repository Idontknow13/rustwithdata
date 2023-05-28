use anyhow::{Context, Result};
use axum::{routing::get, Router};

async fn index() -> &'static str {
    "Hello, RustWithData!"
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/", get(index));

    let addr = "[::]:8080";
    axum::Server::try_bind(&addr.parse()?)
        .context(format!("Failed to bind to address `{addr}`"))?
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
