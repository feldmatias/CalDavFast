use std::env;

pub struct AppConfig {
    pub environment: String,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        Self {
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
        }
    }
}
