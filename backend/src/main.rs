use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    //Build the application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World" }));

    //Define the address (localhost:5000)
    let address = SocketAddr::from(([127, 0, 0, 1], 5000));

    println!("Server is up and running on http://{}", address);

    //Start the server
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
