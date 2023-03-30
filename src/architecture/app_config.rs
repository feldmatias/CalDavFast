use std::env;

pub struct AppConfig {
    pub environment: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}
