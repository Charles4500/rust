use axum::{routing::get, Router};

use crate::handlers::user::{create_user, delete_user, get_user, list_users, update_user};

pub fn user_routes() -> Router<crate::db::Pool> {
    Router::new()
        .route("/", get(|| async { "Hello, World" }))
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/{id}",
            get(get_user).patch(update_user).delete(delete_user),
        )
}
