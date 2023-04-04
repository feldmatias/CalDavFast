use crate::architecture::app_config::AppConfig;
use crate::architecture::mongodb::MongoDb;
use ddi::*;

/*
* Generate the dependency injection container.
* Each injectable service should be registered here.
*/
pub async fn di_container(client: &mongodb::Client, db_name: String) -> DDIProvider {
    let mut services = ServiceCollection::new();
    services.service(Service::new(AppConfig::new()));
    services.service(Service::new(MongoDb::new(client, db_name)));
    /*services.service_factory(|config: &Service<AppConfig>| {
        Ok(Service::new(HelloController::new(config.clone())))
    });*/

    let provider = services.provider();
    DDIProvider::new(provider)
}

pub struct DDIProvider {
    provider: ServiceProvider,
}

impl DDIProvider {
    fn new(provider: ServiceProvider) -> Self {
        DDIProvider { provider }
    }

    pub fn get<T: 'static>(&self) -> &Service<T> {
        self.provider.get::<Service<T>>().unwrap()
    }
}

impl Clone for DDIProvider {
    fn clone(&self) -> Self {
        DDIProvider {
            provider: self.provider.clone(),
        }
    }
}
