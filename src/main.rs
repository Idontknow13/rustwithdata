mod errors;
mod schema;
use schema::User;

use std::time::Duration;

use anyhow::{Context, Result};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};

/// A basic "*hello*" GET endpoint.
async fn index() -> &'static str {
    "Hello, RustWithData!"
}

/// A basic GET endpoint connected to the Users database.
async fn get_user(
    Path(username): Path<String>,
    State(pool): State<PgPool>,
) -> Result<Json<User>, (StatusCode, String)> {
    sqlx::query_as!(
        User,
        "SELECT name, age FROM users WHERE name = $1",
        username
    )
    .fetch_one(&pool)
    .await
    .map(|user| Json(user))
    .map_err(errors::internal_error)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    // enable tracing by default
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // Creating the database connection pool with config from `axum/examples/sqlx-postgres`
    let connstr = std::env::var("DATABASE_URL").context("No DATABASE_URL specified.")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&connstr)
        .await
        .context("Failed to connect to database")?;

    let app = Router::new()
        .route("/", get(index))
        .route("/users/:username", get(get_user))
        .with_state(pool);

    let addr = "[::]:8080";
    axum::Server::try_bind(&addr.parse()?)
        .context(format!("Failed to bind to address `{addr}`"))?
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
