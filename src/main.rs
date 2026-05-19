
use std::env;
use mysql::{Pool, PooledConn};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ConnectionConfig {
    host: String,
    port: u16,
    database: String,

    username: String,
    password: String,
}


fn main() {
    let connection = establish_connection();
}

fn establish_connection() -> PooledConn {
    dotenvy::dotenv().ok();

    let config: ConnectionConfig = envy::from_env().expect("Failed to load config");
    
    let connection_string = format!(
        "mysql://{}:{}@{}:{}/{}", config.username, config.password, config.host, config.port, config.database);

    let pool = Pool::new(connection_string.as_str()).expect("Unable to create pool");
    let conn = pool.get_conn().expect("Unable to create a connection to the pool");

    conn
}
