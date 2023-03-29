use crate::constants::DEFAULT_PORT;
use actix_web::{web, App, HttpServer};
use constants::DEFAULT_RUST_LOG;
use env_logger::Env;
use log::info;
use services::conn::{create_database_connection, create_redis_client};

mod constants;
mod error;
mod handlers;
mod middlewares;
mod models;
mod services;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or(DEFAULT_RUST_LOG));

    let db = web::Data::new(create_database_connection().await);
    let redis = web::Data::new(create_redis_client());
    let port = match std::env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap_or(DEFAULT_PORT),
        _ => DEFAULT_PORT,
    };

    info!("🚀 Service started at port: {} 🚀", port);
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .app_data(redis.clone())
            .wrap(actix_web::middleware::Logger::new("%r %t %s %Dms"))
            .service(handlers::health_checker)
            .service(web::scope("/user").configure(handlers::user::config))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
