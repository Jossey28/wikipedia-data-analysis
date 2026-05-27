use std::{
    env,
    fmt::{self, Display},
    ops::Index,
};

use chrono::Utc;

#[derive(Debug)]
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
            port: env::var("MYSQL_SERVER_PORT")
                .expect("Failed to load MYSQL_SERVER_PORT")
                .parse::<u16>()
                .expect("Failed to parse MYSQL_SERVER_PORT"),
            database: env::var("MYSQL_SERVER_DATABASE")
                .expect("Failed to load MYSQL_SERVER_DATABASE"),
            username: env::var("MYSQL_SERVER_USERNAME")
                .expect("Failed to load MYSQL_SERVER_USERNAME"),
            password: env::var("MYSQL_SERVER_PASSWORD")
                .expect("Failed to load MYSQL_SERVER_PASSWORD"),
        }
    }

    pub fn generate_string(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

#[derive(sqlx::FromRow, Hash, PartialEq, Eq, Debug)]
pub struct Category {
    // https://www.mediawiki.org/wiki/Manual:Category_table
    pub cat_id: u32,
    pub cat_title: Vec<u8>,
    pub cat_pages: i32,
    pub cat_subcats: i32,
    pub cat_files: i32,
}

impl Into<u32> for Category {
    fn into(self) -> u32 {
        self.cat_id
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let title = String::from_utf8(self.cat_title.clone()).unwrap();
        write!(
            f,
            "{} ({}), on {} pages with {} subcategories and {} associated files",
            title, self.cat_id, self.cat_pages, self.cat_subcats, self.cat_files
        )
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct CategoryLinks {
    // https://www.mediawiki.org/wiki/Manual:Categorylinks_table
    pub cl_from: u32, // a "page id" for the category
    pub cl_sortkey: Vec<u8>,
    pub cl_timestamp: chrono::DateTime<Utc>,
    pub cl_sortkey_prefix: Vec<u8>,
    pub cl_type: Vec<u8>,
    pub cl_collation_id: u16,
    pub cl_target_id: u64,
}

impl Display for CategoryLinks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cat_type: CategoryLinksTypes = self.cl_type.clone().into();
        write!(
            f,
            "from cat_id {} ({}) => to cat_id {}",
            self.cl_from, cat_type, self.cl_target_id
        )
    }
}

#[derive(sqlx::Type, Debug)]
#[sqlx(rename_all = "lowercase")]
pub enum CategoryLinksTypes {
    Page,
    Subcat,
    File,
}

impl From<String> for CategoryLinksTypes {
    fn from(value: String) -> Self {
        match value.as_str() {
            "page" => Self::Page,
            "subcat" => Self::Subcat,
            "file" => Self::File,
            unkwn => panic!("unknown cl_type {}", unkwn),
        }
    }
}

impl From<Vec<u8>> for CategoryLinksTypes {
    fn from(value: Vec<u8>) -> Self {
        match String::from_utf8(value).unwrap().as_str() {
            "page" => Self::Page,
            "subcat" => Self::Subcat,
            "file" => Self::File,
            unkwn => panic!("unknown cl_type {}", unkwn),
        }
    }
}

impl Display for CategoryLinksTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::Page => "page",
            Self::File => "file",
            Self::Subcat => "subcat",
        };

        write!(f, "{}", string)
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Page {
    // https://www.mediawiki.org/wiki/Manual:Page_table
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
    // https://www.mediawiki.org/wiki/Manual:Pagelinks_table
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
