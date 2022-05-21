#[cfg(test)]
mod tests {
    use crate::models::work::Work;

    #[test]
    fn it_works() {
        // 28755084
        // 32138716
        // 39123774
        // println!("{:#?}", Work::from_id(28755084).metadata);
        // println!("{:#?}", Work::from_id(39123774).metadata);
        println!("{:#?}", Work::from_id(39120786).metadata);
    }
}

pub mod models;
mod scrape;