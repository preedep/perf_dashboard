use log::info;

fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    info!("Hello, world!");

}