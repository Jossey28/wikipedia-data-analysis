
use std::env;
use mysql::{Pool, PooledConn, prelude::Queryable};

fn main() {
    let connection = establish_connection();
}

fn establish_connection() -> PooledConn {
    dotenvy::dotenv().ok();

    let host = env::var("MYSQL_SERVER_HOST").expect("Unable to load host from .env file");
    let port = env::var("MYSQL_SERVER_PORT").expect("Unable to load port from .env file");
    let username = env::var("MYSQL_SERVER_USERNAME").expect("Unable to load username from .env file");
    let password = env::var("MYSQL_SERVER_PASSWORD").expect("Unable to load password from .env file");
    let database = env::var("MYSQL_SERVER_DATABASE").expect("Unable to load database from .env file");

    let connection_string = format!(
        "mysql://{username}:{password}@{host}:{port}/{database}"
    );

    let pool = Pool::new(connection_string.as_str()).expect("Unable to create pool");
    let conn = pool.get_conn().expect("Unable to create a connection to the pool");

    conn
}
