use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

use crate::types::ConnectionConfig;

pub async fn establish_connection() -> Pool<MySql> {
    let config = ConnectionConfig::new();
    let connection_string = config.generate_string();

    let pool = MySqlPoolOptions::new()
        .max_connections(3)
        .connect(&connection_string)
        .await
        .expect("Failed to open connection");

    pool
}