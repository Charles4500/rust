use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(database_url: &str) {
    let mut connection = PgConnection::establish(database_url)
        .expect("Failed to connect to database for migrations");

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    println!("Migrations completed successfully");
}
