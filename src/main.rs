use cal_dav_fast::server::start_server;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    println!("CalDav Fast");

    start_server().await
}
