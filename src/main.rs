use sqlx::{MySql, QueryBuilder};
use wikipedia_data_analysis::wikipedia_types::Category;
use wikipedia_data_analysis::wikipedia_types::CategoryLinks;
use wikipedia_data_analysis::helpers::establish_connection;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let conn = establish_connection().await;

    let categories: Vec<Category> = sqlx::query_as("
        SELECT cat_id, cat_title, cat_pages, cat_subcats, cat_files
        FROM category 
        LIMIT 32
    ").fetch_all(&conn).await.expect("Failed category SELECT");

    let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
        "SELECT cl_from, cl_sortkey, cl_timestamp, cl_sortkey_prefix, cl_type, cl_collation_id, cl_target_id
        FROM categorylinks WHERE cl_type = '0x737562636174'
        AND cl_target_id in ("
    );

    let mut seperated = query_builder.separated(", ");
    for category in &categories {
        seperated.push(category.cat_id);
    }
    query_builder.push(")");

    let category_links: Vec<CategoryLinks> = query_builder.build_query_as().fetch_all(&conn).await.expect("Failed categorylinks SELECT");

    for cat in &categories {
        println!("{}", cat);
    }

    println!("");

    for link in &category_links {
        println!("{}", link);
    }

    println!("");

    println!("categories: {} \t category links: {}", categories.len(), category_links.len());
}
