mod schema;

use std::time::Duration;

use anyhow::{Context, Result};
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;

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

    // Creating the database connection pool with config from `axum/examples/sqlx-postgres`
    let connstr = "postgresql://postgres:password@rustwithdata_pg/userdb".to_string();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&connstr)
        .await
        .context("Failed to connect to database")?;

    let app = Router::new()
        .route("/", get(index))
        .nest("/users", Router::new().with_state(pool));

    let addr = "[::]:8080";
    axum::Server::try_bind(&addr.parse()?)
        .context(format!("Failed to bind to address `{addr}`"))?
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
