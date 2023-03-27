use log::error;
use redis::Client;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{process::exit, time::Duration};

pub async fn create_database_connection() -> DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        error!("`DATABASE_URL` not found, please set it in you .env file.");
        exit(1)
    });

    let mut option = ConnectOptions::new(database_url);
    option
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    Database::connect(option).await.unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    })
}

pub fn create_redis_client() -> Client {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| {
        error!("`REDIS_URL` not found, please set it in you .env file.");
        exit(1)
    });
    Client::open(redis_url).unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    })
}
