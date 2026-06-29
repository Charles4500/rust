use axum::{routing::post, Router};

use crate::{
    handlers::mpesa::{callback, stk_push},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/pay", post(stk_push))
        .route("/callback", post(callback))
}
