use crate::architecture::app_config::AppConfig;
use crate::architecture::mongodb::MongoDb;
use crate::CalDav;
use ddi::*;

/*
* Generate the dependency injection container.
* Each injectable service should be registered here.
*/
pub fn di_container(mongo: MongoDb) -> ServiceProvider {
    let mut services = ServiceCollection::new();
    services.service(Service::new(AppConfig::new()));
    services.service(Service::new(mongo));
    services
        .service_factory(|mongo: &Service<MongoDb>| Ok(Service::new(CalDav::new(mongo.clone()))));

    services.provider()
}
