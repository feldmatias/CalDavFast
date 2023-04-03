use std::env;

pub struct AppConfig {
    pub environment: String,
    pub server_port: u16,
    pub mongodb: String,
    pub mongo_database: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            environment: env::var("RUST_ENV").unwrap_or_else(|_| "local".to_string()),
            mongodb: env::var("MONGO_DB").expect("MongoDB env var is needed"),
            mongo_database: env::var("MONGO_DB_NAME").unwrap_or_else(|_| "caldav".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()
                .unwrap(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}
