use actix_web::{
    get, middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

use crate::{
    architecture::{
        app_config::AppConfig,
        dependency_injection::{di_container, DDIProvider},
    },
    controllers::hello_controller::HelloController,
};

#[get("/")]
async fn hello(provider: web::Data<DDIProvider>) -> impl Responder {
    let controller = provider.get::<HelloController>();
    let result = controller.hello().await;
    HttpResponse::Ok().json(result)
}

pub async fn start_server() -> Result<(), std::io::Error> {
    let app_config = AppConfig::new();
    let client = mongodb::Client::with_uri_str(&app_config.mongodb)
        .await
        .unwrap();
    let di_provider = di_container(&client, app_config.mongo_database).await;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(di_provider.clone()))
            .service(hello)
    })
    .bind(("127.0.0.1", app_config.server_port))?
    .run()
    .await
}
