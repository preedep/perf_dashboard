use actix_web::{App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv;
use log::info;
use pretty_env_logger;
use perf_dashboard::interface::api;
use actix_files as fs;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    info!("Starting server...");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .configure(api::config)
            .service(fs::Files::new("/", "./statics").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
