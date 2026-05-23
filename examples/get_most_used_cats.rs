use std::cmp::Reverse;
use std::env;
use std::num::{NonZero, NonZeroU64};

use wikipedia_data_analysis::types::Category;

use wikipedia_data_analysis::helpers::establish_connection;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args: Vec<String> = env::args().collect();

    let amount: NonZero<u64> =  match args[1].parse() {
        Ok(n) => n,
        Err(error) => {
            eprintln!("Error {}; supply  non-negative integer. Running with default value 10", error);
            NonZeroU64::new(10).unwrap()
        }        
    };
    
    dotenvy::dotenv().ok();

    let conn = establish_connection().await;

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM category"
    ).fetch_one(&conn).await?;
    let count = count.0;

    let mut rows: Vec<Category> = sqlx::query_as(
        &format!("SELECT cat_id, cat_title, cat_pages, cat_subcats, cat_files
        FROM category
        ORDER BY cat_pages DESC
        LIMIT {}
        ", amount),
    )
    .fetch_all(&conn)
    .await?;

    println!("Top {} / {} most linked categories\n", amount, count);
    
    let min: usize = match amount.get() < count.try_into().unwrap() { // Don't index past amount of rows
        true => amount.get() as usize,
        false => count as usize,
    };

    rows.sort_by_key(|cat| Reverse(cat.cat_pages));
    for row in &rows[..min] {
        println!("{}", row);
    }

    Ok(())
}
