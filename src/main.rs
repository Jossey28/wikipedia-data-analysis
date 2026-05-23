use std::cmp::Reverse;

use wikipedia_data_analysis::types;
use wikipedia_data_analysis::helpers::establish_connection;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();

    let conn = establish_connection().await;

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM category"
    ).fetch_one(&conn)
        .await?;

    let mut rows: Vec<types::Category> = sqlx::query_as(
        "SELECT cat_id, cat_title, cat_pages, cat_subcats, cat_files
        FROM category
        ORDER BY cat_pages DESC
        LIMIT 10
        ",
    )
    .fetch_all(&conn)
    .await?;

    println!("There are {} categories below are the top 10 most linked\n", count.0);
    rows.sort_by_key(|cat| Reverse(cat.cat_pages));
    for row in &rows[..10] {
        println!("{}", row);
    }

    Ok(())
}
