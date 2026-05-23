use std::env;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

pub mod types;

use types::ConnectionConfig;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();

    let conn = establish_connection().await;

    let row: types::Category = sqlx::query_as(
        "SELECT cat_id, cat_title, cat_pages, cat_subcats, cat_files FROM category LIMIT 1",
    )
    .fetch_one(&conn)
    .await?;

    println!("{:#?}", row);

    Ok(())
}

async fn establish_connection() -> Pool<MySql> {

    let config  = ConnectionConfig::new();
    let connection_string = config.generate_string();

    let pool = MySqlPoolOptions::new()
        .max_connections(3)
        .connect(&connection_string)
        .await
        .expect("Failed to open connection");

    pool
}
