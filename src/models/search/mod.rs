use std::{
    collections::{HashSet, VecDeque},
    ops::Range,
};

use typed_builder::TypedBuilder;

mod url_rep;

use crate::{models::search::url_rep::UrlRep, scrape::search::parse_search_results};

use super::{
    language::Language,
    work::{Category, Rating, Warning, WorkMetadata},
};

#[derive(Debug, Clone)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Descending
    }
}

#[derive(Debug, Clone)]
pub enum SortBy {
    BestMatch,
    Author,
    Title,
    DatePosted,
    DateUpdated,
    WordCount,
    Hits,
    Kudos,
    Comments,
    Bookmarks,
}

impl Default for SortBy {
    fn default() -> Self {
        SortBy::BestMatch
    }
}

#[derive(Debug, Default, TypedBuilder, Clone)]
#[builder(field_defaults(default))]
pub struct SearchQuery {
    #[builder(setter(into))]
    pub any: String,
    #[builder(setter(into))]
    pub title: String,
    #[builder(setter(into))]
    pub author: String,
    #[builder(setter(into))]
    pub date: String,
    #[builder(setter(strip_option))]
    pub completed: Option<bool>,
    #[builder(setter(strip_option))]
    pub crossover: Option<bool>,
    #[builder(!default, setter(strip_bool))]
    pub single_chapter: bool,
    #[builder(setter(strip_option))]
    pub word_count: Option<Range<usize>>,
    #[builder(setter(strip_option))]
    pub language: Option<Language>,
    #[builder(setter(into))]
    pub fandoms: HashSet<String>,
    #[builder(setter(strip_option))]
    pub rating: Option<Rating>,
    #[builder(setter(into))]
    pub warnings: HashSet<Warning>,
    #[builder(setter(into))]
    pub categories: HashSet<Category>,
    #[builder(setter(into))]
    pub characters: HashSet<String>,
    #[builder(setter(into))]
    pub relationships: HashSet<String>,
    #[builder(setter(into))]
    pub tags: HashSet<String>,
    #[builder(setter(strip_option))]
    pub hits: Option<Range<usize>>,
    #[builder(setter(strip_option))]
    pub kudos: Option<Range<usize>>,
    #[builder(setter(strip_option))]
    pub comments: Option<Range<usize>>,
    #[builder(setter(strip_option))]
    pub bookmarks: Option<Range<usize>>,
    pub sort_by: SortBy,
    pub sort_direction: SortDirection,
    #[builder(setter(skip))]
    search_results: VecDeque<WorkMetadata>,
    #[builder(setter(skip))]
    page: usize,
}

impl SearchQuery {
    fn url(&self) -> String {
        fn multi_prop<T: UrlRep>(v: &HashSet<T>, prop: &str) -> String {
            v.iter()
                .map(|w| format!("work_search[{prop}][]={}", w.url()))
                .collect::<Vec<String>>()
                .join("&")
        }

        format!(
            include_str!("search_url.txt"),
            self.page,
            self.any,
            self.title,
            self.author,
            self.date,
            self.completed.url(),
            self.crossover.url(),
            self.single_chapter as u8,
            self.word_count.url(),
            self.language.url(),
            self.fandoms.url(),
            self.rating.url(),
            multi_prop(&self.warnings, "archive_warning_ids"),
            multi_prop(&self.categories, "category_ids"),
            self.characters.url(),
            self.relationships.url(),
            self.tags.url(),
            self.hits.url(),
            self.kudos.url(),
            self.comments.url(),
            self.bookmarks.url(),
            self.sort_by.url(),
            self.sort_direction.url()
        )
        .replace(' ', "+")
    }
}

impl Iterator for SearchQuery {
    type Item = WorkMetadata;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.search_results.pop_front();

        if next.is_none() {
            let html = reqwest::blocking::get(self.url()).unwrap().text().unwrap();

            self.page += 1;

            let results = parse_search_results(&html);

            self.search_results.append(&mut VecDeque::from(results));

            self.search_results.pop_front()
        } else {
            next
        }
    }
}
