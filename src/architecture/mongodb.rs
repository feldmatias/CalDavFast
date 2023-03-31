use crate::architecture::app_config::AppConfig;
use mongodb::Client;

pub struct MongoDb {
    pub client: Client,
}

impl MongoDb {
    pub async fn new() -> Self {
        let config = AppConfig::new();
        let client = Client::with_uri_str(&config.mongodb).await.unwrap();
        Self { client }
    }
}
