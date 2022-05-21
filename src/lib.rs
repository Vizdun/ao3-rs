#[cfg(test)]
mod tests {
    use crate::models::work::Work;

    #[test]
    fn it_works() {
        // 28755084
        // 32138716
        println!("{:#?}", Work::from_id(28755084).metadata);
    }
}

pub mod models;
mod scrape;