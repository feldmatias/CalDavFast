use ddi::Service;
use serde::{Deserialize, Serialize};

use crate::architecture::app_config::AppConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloResponse {
    pub environment: String,
}
pub struct HelloController {
    config: Service<AppConfig>,
}

impl HelloController {
    pub fn new(config: Service<AppConfig>) -> Self {
        HelloController { config }
    }
}

impl HelloController {
    pub async fn hello(&self) -> HelloResponse {
        let env = self.config.environment.to_string();
        HelloResponse { environment: env }
    }
}
