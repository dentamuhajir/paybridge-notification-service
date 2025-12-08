use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    // Database connection string
    pub database_url: String,

    // Server host and port
    pub app_host: String,
    pub app_port: u16,
}

impl Config {
    pub fn load() -> Self {
        // ----------------------------------------------------------
        // Load variables from `.env` file into environment.
        // If the file doesn't exist, it's okay (e.g., in production).
        // ----------------------------------------------------------
        dotenv().ok();

        // ----------------------------------------------------------
        // Required config:
        //
        // If DATABASE_URL is missing, the app cannot run.
        // `expect(...)` will stop the program with a clear error message.
        // ----------------------------------------------------------
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env or environment");

        // ----------------------------------------------------------
        // Optional configs with default values:
        //
        // APP_HOST:
        //   - If missing, default to "0.0.0.0" so the service is reachable externally.
        //
        // unwrap_or_else(|_| ...) is used so if the env variable does
        // not exist, the closure returns the default value.
        // ----------------------------------------------------------
        let app_host = env::var("APP_HOST")
            .unwrap_or_else(|_| "0.0.0.0".into());

        // ----------------------------------------------------------
        // APP_PORT:
        //   - Reads environment variable as string.
        //   - Defaults to "8081" if missing.
        //   - parse() converts it to u16.
        //   - expect(...) ensures the number is valid.
        //
        // This is similar to Java:
        // Integer.parseInt(System.getenv().getOrDefault("APP_PORT", "8081"))
        // ----------------------------------------------------------
        let app_port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8081".into())
            .parse()
            .expect("APP_PORT must be a valid u16 number");

        // Construct the Config struct
        Self {
            database_url,
            app_host,
            app_port,
        }
    }

    // ----------------------------------------------------------
    // Helper getters
    // ----------------------------------------------------------

    pub fn host(&self) -> String {
        self.app_host.clone()
    }

    pub fn port(&self) -> u16 {
        self.app_port
    }
}
