mod migrations;

pub use migrations::run_migrations;

use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn create_pool(database_url: &str) -> Pool {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder()
        .build(config)
        .await
        .expect("Failed to create pool")
}
