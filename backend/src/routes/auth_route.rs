use axum::{routing::post, Router};

use crate::handlers::auth::{login, register};
use crate::state::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("'/v1/api/auth/register", post(register))
        .route("/v1/api/auth/login", post(login))
}
