use crate::architecture::app_config::AppConfig;
use ddi::*;

struct TestService {
    config: Service<AppConfig>,
}

impl TestService {
    pub fn print(&self) {
        println!("from test service {}", self.config.environment);
    }
}

/*
* Generate the dependency injection container.
* Each injectable service should be registered here.
*/
pub fn di_container() -> ServiceProvider {
    let mut services = ServiceCollection::new();
    services.service(Service::new(AppConfig::new()));
    services.service_factory(|config: &Service<AppConfig>| {
        Ok(Service::new(TestService {
            config: config.clone(),
        }))
    });

    let provider = services.provider();
    let test_service = provider.get::<Service<TestService>>().unwrap();
    test_service.print();
    provider
}
