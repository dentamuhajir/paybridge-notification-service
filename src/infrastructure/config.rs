use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok(); // Load .env file if present

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env or environment");

        Self { database_url }
    }
}