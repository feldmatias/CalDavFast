use cal_dav_fast::architecture::dependency_injection::di_container;
use cal_dav_fast::architecture::mongodb::MongoDb;
use cal_dav_fast::CalDav;
use ddi::{Service, ServiceResolverExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    println!("Hello, world! from rust");

    // Need to create mongo outside di because of async/await
    let mongo = MongoDb::new().await;
    let di_provider = di_container(mongo);

    let caldav = di_provider.get::<Service<CalDav>>().unwrap();

    caldav.run().await
}
