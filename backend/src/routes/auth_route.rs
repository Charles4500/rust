use axum::{routing::post, Router};

use crate::handlers::auth::{login, register};
use crate::state::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}
