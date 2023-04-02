use cal_dav_fast::CalDav;
use ddi::{Service, ServiceProvider, ServiceResolverExt};

mod common;
use common::*;

#[test]
fn calculate_doubles_age3() {
    run_test(|provider: &ServiceProvider| {
        Box::pin(async {
            let app = provider.get::<Service<CalDav>>().unwrap();

            let result = app.run().await;
            assert_eq!(1, 1);
            assert!(result.is_ok());
        })
    });
}

#[test]
fn calculate_doubles_age5() {
    run_test(|provider: &ServiceProvider| {
        Box::pin(async {
            let app = provider.get::<Service<CalDav>>().unwrap();

            let result = app.run().await;
            assert_eq!(1, 1);
            assert!(result.is_ok());
        })
    });
}

#[test]
fn calculate_doubles_age4() {
    run_test(|provider: &ServiceProvider| {
        Box::pin(async {
            let app = provider.get::<Service<CalDav>>().unwrap();

            let result = app.run().await;
            assert_eq!(1, 1);
            assert!(result.is_ok());
        })
    });
}
