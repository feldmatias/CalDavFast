/*use cal_dav_fast::architecture::dependency_injection::{di_container, DDIProvider};
use cal_dav_fast::architecture::mongodb::MongoDb;
use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

static TESTS_RUNTIME: Lazy<(Runtime, mongodb::Client)> = Lazy::new(|| {
    dotenv::from_filename(".env.test").ok();
    let runtime = Runtime::new().unwrap();
    let mongo_client = runtime.block_on(async {
        let app_config = cal_dav_fast::architecture::app_config::AppConfig::new();
        let mongo = mongodb::Client::with_uri_str(&app_config.mongodb)
            .await
            .unwrap();
        mongo
    });
    (runtime, mongo_client)
});

fn test_id() -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static ID: AtomicUsize = AtomicUsize::new(0);
    ID.fetch_add(1, Ordering::SeqCst)
}

pub fn run_test(test: fn(&DDIProvider) -> BoxFuture<()>) -> () {
    let (runtime, client) = &*TESTS_RUNTIME;
    runtime.block_on(async {
        let db_name = format!("caldav_test{}", test_id());
        let provider = di_container(&client, db_name).await;

        setup(&provider).await;

        test(&provider).await;

        teardown(&provider).await;
    });
}

async fn setup(provider: &DDIProvider) {
    // Could not find a way to catch panics in async functions
    teardown(provider).await;
}

async fn teardown(provider: &DDIProvider) {
    provider.get::<MongoDb>().db.drop(None).await.unwrap();
}*/
