use std::fs::File;

use anyhow::Ok;
use parse_mediawiki_sql::{
    iterate_sql_insertions,
    schemas::Page,
    field_types::{PageNamespace, PageTitle},
    utils::memory_map,
};

fn main() -> anyhow::Result<()> {
    let page_sql = unsafe { memory_map("data\\enwiki-latest-page.sql")? };
    let redirects: Vec<(PageNamespace, PageTitle)> = iterate_sql_insertions(&page_sql).filter_map(
        |Page { namespace, title, is_redirect, ..}| {
            if is_redirect {
                Some((namespace, title))
            } else {
                None
            }
        },
    ).collect();

    println!("redirects vec {:?}", redirects);
    Ok(())
}
