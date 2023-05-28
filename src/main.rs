//! # Rust with Data
//!
//! A barebones and minimal implementation of a Create-Read
//! API connected to a database through another container
//! instance.
//!

mod errors;
mod schema;
use schema::User;

use std::time::Duration;

use anyhow::{Context, Result};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};

/// A basic "*hello*" GET endpoint.
async fn index() -> &'static str {
    "Hello, RustWithData! Please use the /users endpoint for more interactibility."
}

/// A basic GET endpoint connected to the Users database.
async fn get_user(
    Path(username): Path<String>,
    State(pool): State<PgPool>,
) -> Result<Json<User>, (StatusCode, String)> {
    sqlx::query_as::<_, User>("SELECT name, age FROM users WHERE name = $1")
        .bind(username)
        .fetch_one(&pool)
        .await
        .map(|user| Json(user))
        .map_err(errors::internal_error)
}

/// A basic POST endpoint connected to the Users database.
async fn add_user(
    State(pool): State<PgPool>,
    Json(userdata): Json<User>,
) -> Result<Json<User>, (StatusCode, String)> {
    sqlx::query_as::<_, User>("INSERT INTO users(name, age) VALUES ($1, $2) RETURNING name, age")
        .bind(userdata.name)
        .bind(userdata.age)
        .fetch_one(&pool)
        .await
        .map(|user| Json(user))
        .map_err(errors::internal_error)
}

#[tokio::main]
async fn main() -> Result<()> {
    // does not trigger when ran inside the container; uses docker-compose ENV
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
        .route("/users", post(add_user))
        .route("/users/:username", get(get_user))
        .with_state(pool);

    let addr = "[::]:8080";
    axum::Server::try_bind(&addr.parse()?)
        .context(format!("Failed to bind to address `{addr}`"))?
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
