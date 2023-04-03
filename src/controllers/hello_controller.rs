use ddi::Service;
use serde::{Deserialize, Serialize};

use crate::architecture::app_config::AppConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct XMLResponse {
    pub environment: String,
    pub internal: Internal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Internal {
    #[serde(rename = "@environment")]
    pub environment: String,
    pub environment2: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XMLRequest {
    pub environment: String,

    #[serde(rename = "$value")]
    pub body: String,
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
    pub async fn hello(&self, request: XMLRequest) -> XMLResponse {
        let env = self.config.environment.to_string();
        XMLResponse {
            environment: request.environment,
            internal: Internal {
                environment: request.body,
                environment2: env,
            },
        }
    }
}
