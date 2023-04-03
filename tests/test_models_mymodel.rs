use cal_dav_fast::{architecture::dependency_injection::DDIProvider, CalDav};

mod common;
use common::*;

#[test]
fn calculate_doubles_age3() {
    run_test(|provider: &DDIProvider| {
        Box::pin(async {
            let app = provider.get::<CalDav>();

            let result = app.run().await;
            assert_eq!(1, 1);
            assert!(result.is_ok());
        })
    });
}

#[test]
fn calculate_doubles_age5() {
    run_test(|provider: &DDIProvider| {
        Box::pin(async {
            let app = provider.get::<CalDav>();

            let result = app.run().await;
            assert_eq!(1, 1);
            assert!(result.is_ok());
        })
    });
}

#[test]
fn calculate_doubles_age4() {
    run_test(|provider: &DDIProvider| {
        Box::pin(async {
            let app = provider.get::<CalDav>();

            let result = app.run().await;
            assert_eq!(1, 1);
            assert!(result.is_ok());
        })
    });
}
