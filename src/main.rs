use cal_dav_fast::architecture::app_config::AppConfig;
use cal_dav_fast::architecture::dependency_injection::di_container;
use cal_dav_fast::CalDav;
use ddi::{Service, ServiceResolverExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    println!("Hello, world! from rust");

    let app_config = AppConfig::new();
    let client = mongodb::Client::with_uri_str(&app_config.mongodb)
        .await
        .unwrap();
    let di_provider = di_container(&client, "caldav".to_string()).await;

    let caldav = di_provider.get::<Service<CalDav>>().unwrap();

    caldav.run().await
}
