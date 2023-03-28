mod app_config;

fn main() {
    println!("Hello, world! from rust");

    let config = app_config::AppConfig::new();
    println!("environment: {}", config.environment);

    cal_dav_fast::print_model();
}
