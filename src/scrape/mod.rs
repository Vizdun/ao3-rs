pub mod search;
pub mod work;

use scraper::{element_ref::Select, ElementRef, Selector};
pub use work::*;

use crate::models::work::{NamedAuthor, Rating, Warning, WorkAuthor};

pub fn parse_id(e: ElementRef) -> u32 {
    e.value().attr("href").unwrap()[7..]
        .split('/')
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

pub fn parse_authors(e: Select) -> Vec<WorkAuthor> {
    let v: Vec<WorkAuthor> = e
        .map(|a| {
            let username = a.value().attr("href").unwrap()[7..]
                .split('/')
                .next()
                .unwrap()
                .to_string();

            let pseud = a.value().attr("href").unwrap()[7..]
                .splitn(3, '/')
                .nth(2)
                .unwrap()
                .to_string();

            if username == "orphan_account" {
                WorkAuthor::OrphanAccount
            } else {
                WorkAuthor::Named({
                    NamedAuthor {
                        pseud: if pseud == username { None } else { Some(pseud) },
                        username,
                    }
                })
            }
        })
        .collect();

    if v.is_empty() {
        vec![WorkAuthor::Anonymous]
    } else {
        v
    }
}

pub fn parse_summary(fragment: ElementRef) -> Option<String> {
    let summary_selector = Selector::parse("blockquote.userstuff").unwrap();

    fragment.select(&summary_selector).next().map(|n| {
        n.text()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join("\n")
            .trim()
            .to_string()
    })
}

pub fn parse_rating(e: ElementRef) -> Rating {
    e.text().next().unwrap().try_into().unwrap()
}

pub fn parse_warnings(e: Select) -> Vec<Warning> {
    e.map(|a| a.text().collect::<String>().as_str().try_into().unwrap())
        .collect()
}

pub fn parse_tags(e: Select) -> Vec<String> {
    e.map(|a| a.text().collect()).collect()
}

macro_rules! parse_try_into {
    ($e:expr) => {
        $e.next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .trim()
            .try_into()
            .unwrap()
    };
}

pub(crate) use parse_try_into;

macro_rules! parse_parse {
    ($e:expr) => {
        $e.next()
            .map(|e| e.text().next().unwrap().to_string().parse().unwrap())
            .unwrap_or(0)
    };
}

pub(crate) use parse_parse;
