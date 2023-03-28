use crate::dependency_injection::di_container;

mod app_config;
mod dependency_injection;

fn main() {
    println!("Hello, world! from rust");

    let config = app_config::AppConfig::new();
    println!("environment: {}", config.environment);

    di_container();

    cal_dav_fast::print_model();
}
