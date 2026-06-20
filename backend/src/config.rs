use std::env;

pub struct Config {
    pub database_url: String,
    pub client_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            client_url: env::var("CLIENT_URL").expect("CLIENT_URL must be set"),
        }
    }
}
