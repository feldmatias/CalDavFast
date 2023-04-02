use std::env;

pub struct AppConfig {
    pub environment: String,
    pub mongodb: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            environment: env::var("RUST_ENV").unwrap_or_else(|_| "local".to_string()),
            mongodb: env::var("MONGO_DB").expect("MongoDB env var is needed"),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}
