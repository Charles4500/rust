mod common;
mod config;
mod db;
mod dto;
mod handlers;
mod models;
mod routes;
mod schema;

use axum::{http::Method, Router};
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let config = config::Config::from_env();

    // Run migrations
    db::run_migrations(&config.database_url);

    // Setup connection pool
    let pool = db::create_pool(&config.database_url).await;

    let origin = config.client_url.parse().unwrap();

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
        .merge(routes::user_routes::user_routes())
        .layer(cors)
        .with_state(pool);

    // Start server
    let address = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("Server is up and running on http://{}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
