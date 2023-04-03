use actix_web::{
    get, middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use cal_dav_fast::architecture::dependency_injection::di_container;
use cal_dav_fast::architecture::{app_config::AppConfig, dependency_injection::DDIProvider};

#[get("/")]
async fn hello(provider: web::Data<DDIProvider>) -> impl Responder {
    let config = provider.get::<AppConfig>();
    HttpResponse::Ok().body(config.environment.to_string())
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    println!("CalDav Fast");

    let app_config = AppConfig::new();
    let client = mongodb::Client::with_uri_str(&app_config.mongodb)
        .await
        .unwrap();
    let di_provider = di_container(&client, "caldav".to_string()).await;

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
