use std::{env, fmt};

use chrono::Utc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConnectionConfig {
    host: String,
    port: u16,
    database: String,

    username: String,
    password: String,
}

impl ConnectionConfig {
    pub fn new() -> Self {
        Self {
            host: env::var("MYSQL_SERVER_HOST").expect("Failed to load MYSQL_SERVER_HOST"),
            port: env::var("MYSQL_SERVER_PORT").expect("Failed to load MYSQL_SERVER_PORT").parse::<u16>().expect("Failed to parse MYSQL_SERVER_PORT"),
            database: env::var("MYSQL_SERVER_DATABASE").expect("Failed to load MYSQL_SERVER_DATABASE"),
            username: env::var("MYSQL_SERVER_USERNAME").expect("Failed to load MYSQL_SERVER_USERNAME"), 
            password: env::var("MYSQL_SERVER_PASSWORD").expect("Failed to load MYSQL_SERVER_PASSWORD"),
        }
    }

    pub fn generate_string(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Category {
    pub cat_id: u32,
    pub cat_title: Vec<u8>,
    pub cat_pages: i32,
    pub cat_subcats: i32,
    pub cat_files: i32,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let title = String::from_utf8(self.cat_title.clone()).unwrap();
        write!(f, "{} ({}), on {} pages with {} subcategories and {} associated files", title, self.cat_id, self.cat_pages, self.cat_subcats, self.cat_files)
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct CategoryLinks {
    cl_from: u32,
    cl_sortkey: Vec<u8>,
    cl_timestamp: chrono::DateTime<Utc>,
    cl_sortkey_prefix: Vec<u8>,
    cl_type: CategoryLinksTypes,
    cl_collation_id: u16,
    cl_target_id: u64,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "cl_type", rename_all = "lowercase")]
pub enum CategoryLinksTypes {
    Page,
    Subcat,
    File,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Page {
    page_id: u32,
    page_namespace: i32,
    page_title: Vec<u8>,
    page_is_redirect: u8,
    page_is_new: u8,
    page_random: f64,
    page_touched: Vec<u8>,
    page_links_update: Option<Vec<u8>>,
    page_latest: u32,
    page_len: u32,
    page_content_model: Option<Vec<u8>>,
    page_lang: Option<Vec<u8>>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct PageLinks {
    pl_from: u32,
    pl_from_namespace: i32,
    pl_target_id: u64,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Redirect {
    rd_from: u32,
    rd_namespace: i32,
    rd_title: Vec<u8>,
    rd_interwiki: Option<Vec<u8>>,
    rd_fragment: Option<Vec<u8>>,
}
