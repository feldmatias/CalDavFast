use async_trait::async_trait;
use cal_dav_fast::architecture::dependency_injection::di_container;
use cal_dav_fast::architecture::mongodb::MongoDb;
use ddi::{Service, ServiceProvider};
use lazy_static::lazy_static;
use std::sync::Arc;
use test_context::AsyncTestContext;

/*lazy_static! {
    static ref TESTS_MONGO_PROVIDER : AsyncOnce<MongoDb> = AsyncOnce::new(async{
        dotenv::from_filename(".env.test").ok();
        let mongo = MongoDb::new().await;
            mongo
       });
}*/

pub struct TestsContext {
    mongo: MongoDb,
}

impl TestsContext {
    pub async fn get_provider(&self) -> ServiceProvider {
        let container = di_container(self.mongo.clone()).await;
        container
    }
}

#[async_trait]
impl AsyncTestContext for TestsContext {
    async fn setup() -> TestsContext {
        dotenv::from_filename(".env.test").ok();
        let mongo = MongoDb::new().await;
        TestsContext {
            mongo: mongo.clone(),
        }
    }

    async fn teardown(self) {
        // Perform any teardown you wish.
        self.mongo
            .client
            .database("caldav_test")
            .drop(None)
            .await
            .unwrap();
    }
}
