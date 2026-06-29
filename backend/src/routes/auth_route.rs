use axum::{routing::get, Router};

use crate::{handlers::auth::me, state::AppState};

pub fn auth_routes() -> Router<AppState> {
    Router::new().route("/me", get(me))
}
