mod common;
mod configs;
mod db;
mod dto;
mod handlers;
mod models;
mod routes;
mod schema;
mod state;
mod utils;
use axum::{http::Method, Router};

use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // let config = config::Config::from_env();
    let db_config = configs::db_config::Config::from_env();
    let google_client = configs::auth::get_google_oauth_client(None, None, None);
    // Run migrations
    db::run_migrations(&db_config.database_url);

    // Setup connection pool
    let pool = db::create_pool(&db_config.database_url).await;

    let state = AppState {
        db_pool: pool,
        jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        google_client,
    };

    let origin = db_config.client_url.parse().unwrap();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact(origin))
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
        ]);

    // 2. Mount Routes
    let app = Router::new()
        .merge(routes::test_route::test_route())
        .merge(routes::auth_route::auth_routes())
        .layer(cors)
        .with_state(state);

    // Start server
    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server is up and running on http://{}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
