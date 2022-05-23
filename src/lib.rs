#[cfg(test)]
mod tests {
    use crate::models::{
        search::SearchQuery,
        work::{Work, WorkMetadata},
    };

    #[test]
    fn it_works() {
        // 28755084
        // 32138716
        // 39123774
        // println!("{:#?}", Work::from_id(28755084).metadata);
        // println!("{:#?}", Work::from_id(39123774).metadata);
        // println!("{:#?}", Work::from_id(39120786).metadata);

        println!(
            "{:#?}",
            SearchQuery::builder()
                .build()
                .collect::<Vec<WorkMetadata>>()
        )
    }
}

mod models;
mod scrape;

pub use models::*;