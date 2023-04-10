use actix_web::{middleware, web::Data, App, HttpServer};

use crate::architecture::{app_config::AppConfig, dependency_injection::di_container};

/*#[post("/")]
async fn hello(provider: web::Data<DDIProvider>, request: Xml<XMLRequest>) -> impl Responder {
    provider.get::<HelloController>().hello(request.0).await
}*/

pub async fn start_server() -> Result<(), std::io::Error> {
    let app_config = AppConfig::new();
    let client = mongodb::Client::with_uri_str(&app_config.mongodb).await.unwrap();
    let di_provider = di_container(&client, app_config.mongo_database).await;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(di_provider.clone()))
        //.service(hello)
    })
    .bind(("127.0.0.1", app_config.server_port))?
    .run()
    .await
}
