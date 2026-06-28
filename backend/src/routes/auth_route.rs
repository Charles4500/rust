use axum::{routing::post, Router};

use crate::handlers::auth::{login, register};
use crate::state::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
