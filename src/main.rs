use std::collections::HashMap;

use sqlx::{MySql, QueryBuilder};
use wikipedia_data_analysis::helpers::establish_connection;
use wikipedia_data_analysis::wikipedia_types::Category;
use wikipedia_data_analysis::wikipedia_types::CategoryLinks;
use wikipedia_data_analysis::wikipedia_types::CategoryUsingPageId;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let conn = establish_connection().await;

    let category_map: HashMap<u32, Category> = {
        let categories: Vec<CategoryUsingPageId> = sqlx::query_as( // Starting from page p because its indexed
            "
        SELECT p.page_id, c.cat_id, CONVERT(c.cat_title USING utf8mb4) AS cat_title, c.cat_pages, c.cat_subcats, c.cat_files
        FROM page p
        JOIN category c ON p.page_title = c.cat_title 
        WHERE p.page_namespace = 14
        ORDER BY c.cat_subcats DESC
    ",
        )
        .fetch_all(&conn)
        .await
        .expect("Failed category SELECT");
        let mut tmp: HashMap<u32, Category> = HashMap::with_capacity(categories.len());
        for cat in categories {
            tmp.insert(cat.page_id, cat.cat);
        }
        tmp
    };

    let family: HashMap<u32, Vec<u32>> = {
        let subcat_list: Vec<CategoryLinks> = sqlx::query_as(
  "SELECT cl_from, cl_sortkey, cl_timestamp, cl_sortkey_prefix, cl_type, cl_collation_id, cl_target_id
        FROM categorylinks 
        WHERE cl_target_id > 0
        AND cl_type = 'subcat'
    ").fetch_all(&conn).await.expect("Failed categorylinks query");

        let mut tmp: HashMap<u32, Vec<u32>> = HashMap::with_capacity(subcat_list.len());

        for subcat in subcat_list {
            if category_map.contains_key(&subcat.cl_from)
                & category_map.contains_key(&(subcat.cl_target_id as u32))
            {
                // if its a valid category -> category relationship ; also prevents deleted relationships from popping up
                tmp.entry(subcat.cl_target_id as u32) // Add its "parent" to the this map
                    .or_default()
                    .push(subcat.cl_from); // and add itself into the vector of kids
            }
        }
        tmp
    };

    let edges: usize = family.values().map(|v| v.len()).sum();
    println!("nodes: {}, children: {}", family.len(), edges)
}
