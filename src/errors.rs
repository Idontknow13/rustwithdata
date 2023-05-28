//! This submodule contains the necessary map_err functions
//! for the application.

use std::error::Error;

use axum::http::StatusCode;

pub fn internal_error<E: Error>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
