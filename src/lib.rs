#[cfg(test)]
mod tests {
    pub use {crate::enums::*, crate::structs::*};

    #[test]
    fn it_works() {
        // 28755084
        // 32138716
        println!("{:#?}", Work::from_id(28755084));
    }
}

mod enums;
mod structs;
pub use {enums::*, structs::*};

mod scrape;