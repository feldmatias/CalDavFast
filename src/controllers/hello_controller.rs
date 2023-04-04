use actix_web::{http::header::ContentType, HttpResponse, Responder};
use ddi::Service;
use serde::{Deserialize, Serialize};
use serde_xml_rs::to_string;

use crate::architecture::app_config::AppConfig;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "xml-response")]
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
    pub async fn hello(&self, request: XMLRequest) -> impl Responder {
        let env = self.config.environment.to_string();
        let response = XMLResponse {
            // This would be in a service
            environment: request.environment,
            internal: Internal {
                environment: request.body,
                environment2: env,
            },
        };
        HttpResponse::Ok()
            .content_type(ContentType::xml())
            .body(to_string(&response).unwrap())
    }
}
