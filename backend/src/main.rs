mod common;
mod config;
mod db;
mod dto;
mod handlers;
mod models;
mod routes;
mod schema;

use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let config = config::Config::from_env();

    // Run migrations
    db::run_migrations(&config.database_url);

    // Setup connection pool
    let pool = db::create_pool(&config.database_url).await;

    // 2. Mount Routes
    let app = Router::new()
        .merge(routes::user_routes::user_routes())
        .with_state(pool);

    // Start server
    let address = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("Server is up and running on http://{}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
