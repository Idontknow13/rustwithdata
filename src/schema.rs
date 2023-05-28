//! This submodule contains the schema used for the main
//! iteration of this application.

use serde::{Deserialize, Serialize};

/// A basic User schema used for this app.
/// It should match the columns of the database to be used.
///
/// ## JSON Schema
/// ```json
/// {
///     "name": "...",
///     "age": 0
/// }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub name: String,
    pub age: i32,
}
