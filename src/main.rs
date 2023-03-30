use cal_dav_fast::architecture::app_config::AppConfig;
use cal_dav_fast::architecture::dependency_injection::di_container;

fn main() {
    dotenv::dotenv().ok();
    println!("Hello, world! from rust");

    let config = AppConfig::new();
    println!("environment: {}", config.environment);

    di_container();

    cal_dav_fast::print_model();
}
