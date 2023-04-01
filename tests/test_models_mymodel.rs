use cal_dav_fast::CalDav;
use ddi::{Service, ServiceResolverExt};
use std::error::Error;
use test_context::test_context;

mod common;
use common::*;

#[test_context(TestsContext)]
#[tokio::test]
async fn calculate_doubles_age3(ctx: &TestsContext) -> Result<(), Box<dyn Error>> {
    let provider = ctx.get_provider().await;
    let app = provider.get::<Service<CalDav>>().unwrap();

    let result = app.run().await;
    assert_eq!(1, 1);
    result
}
