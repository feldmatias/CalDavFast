use std::env;

pub struct AppConfig {
    pub environment: String,
    pub server_port: u16,
    pub mongodb: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            environment: env::var("RUST_ENV").unwrap_or_else(|_| "local".to_string()),
            mongodb: env::var("MONGO_DB").expect("MongoDB env var is needed"),
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
