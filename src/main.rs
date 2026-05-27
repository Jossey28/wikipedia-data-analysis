use std::collections::HashMap;

use sqlx::{MySql, QueryBuilder};
use wikipedia_data_analysis::helpers::establish_connection;
use wikipedia_data_analysis::wikipedia_types::Category;
use wikipedia_data_analysis::wikipedia_types::CategoryLinks;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let conn = establish_connection().await;

    let category_map: HashMap<u32, Category> = {
        let categories: Vec<Category> = sqlx::query_as(
            "
        SELECT cat_id, cat_title, cat_pages, cat_subcats, cat_files
        FROM category
        ORDER BY cat_subcats DESC
    ",
        )
        .fetch_all(&conn)
        .await
        .expect("Failed category SELECT");

        let mut tmp: HashMap<u32, Category> = HashMap::with_capacity(categories.len());
        for cat in categories {
            tmp.insert(cat.cat_id, cat);
        }

        tmp
    };

    let children: HashMap<u32, Vec<u32>> = {
        let subcat_list: Vec<CategoryLinks> = sqlx::query_as(
        "SELECT cl_from, cl_sortkey, cl_timestamp, cl_sortkey_prefix, cl_type, cl_collation_id, cl_target_id
        FROM categorylinks WHERE cl_type = 'subcat'
    ").fetch_all(&conn).await.expect("Failed categorylinks query");

        let mut tmp: HashMap<u32, Vec<u32>> = HashMap::with_capacity(subcat_list.len());

        for subcat in subcat_list {
            tmp.entry(subcat.cl_from)
                .or_default()
                .push(subcat.cl_target_id as u32);
        }

        tmp
    };

    let total_parents = children.len();
    let total_children: usize = children.values().map(|v| v.len()).sum();
    println!("parents: {} ; children: {}", total_parents, total_children);

    for (parent, kids) in children.iter().take(5) {
        // print!("parent: {} -> ", category_map.get(parent).expect("Unable to find parent in category map"));
        for kid in kids {
            print!("{} ", category_map.get(kid).expect("Unable to find kid in category map "));
        }
    }
}
