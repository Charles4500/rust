use axum::{routing::get, Router};

use crate::state::AppState;

pub fn test_route() -> Router<AppState> {
    Router::new().route("/", get(|| async { "Hello, World" }))
}
