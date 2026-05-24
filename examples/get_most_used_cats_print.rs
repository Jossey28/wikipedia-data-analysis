use std::cmp::Reverse;
use std::env;
use std::num::{NonZero, NonZeroU64};

use clap::Parser;

use wikipedia_data_analysis::helpers::establish_connection;
use wikipedia_data_analysis::wikipedia_types::Category;

// https://docs.rs/clap/latest/clap/ ; Added to get accustomed to cli arg parsing
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// How many rows to output; Non-Zero Unsigned Integer
    #[arg(short, long, default_value_t = NonZeroU64::new(10).unwrap())]
    amount: NonZero<u64>,

    /// Enable to show least used categories on top
    #[arg(short, long, default_value_t = false)]
    reverse: bool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args = Args::parse();
    let amount = args.amount.get();
    let reverse = args.reverse;

    // if args.amount.get() < 1 {
    //         println!("Error; supply  non-negative integer. Running with default value 10");
    //     }

    dotenvy::dotenv().ok();

    let conn = establish_connection().await;

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM category")
        .fetch_one(&conn)
        .await?;
    let count = count.0;

    let mut rows: Vec<Category> = sqlx::query_as(&format!(
        "SELECT cat_id, cat_title, cat_pages, cat_subcats, cat_files
        FROM category
        ORDER BY cat_pages {}
        LIMIT {}
        ",
        if reverse { "ASC" } else { "DESC" },
        amount
    ))
    .fetch_all(&conn)
    .await?;

    if !reverse {
        println!("top {} / {} most linked categories\n", amount, count);
    } else {
        println!("lowest {} / {} most linked categories\n", amount, count);
    }

    let min: usize = match amount < count.try_into().unwrap() {
        // Don't index past amount of rows
        true => amount as usize,
        false => count as usize,
    };

    rows.sort_by_key(|cat| Reverse(cat.cat_pages));
    for row in &rows[..min] {
        println!("{}", row);
    }

    Ok(())
}
