use crate::architecture::app_config::AppConfig;
use crate::architecture::mongodb::MongoDb;
use crate::CalDav;
use ddi::*;

/*
* Generate the dependency injection container.
* Each injectable service should be registered here.
*/
pub async fn di_container(client: &mongodb::Client, db_name: String) -> ServiceProvider {
    let mut services = ServiceCollection::new();
    services.service(Service::new(AppConfig::new()));
    services.service(Service::new(MongoDb::new(client, db_name)));
    services
        .service_factory(|mongo: &Service<MongoDb>| Ok(Service::new(CalDav::new(mongo.clone()))));

    services.provider()
}
