use crate::architecture::app_config::AppConfig;
use mongodb::Client;

pub struct MongoDb {
    pub client: Client,
}

impl MongoDb {
    pub async fn new() -> Self {
        let app_config = AppConfig::new();
        let client = Client::with_uri_str(&app_config.mongodb).await.unwrap();
        Self { client }
    }
}

impl Clone for MongoDb {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
        }
    }
}
