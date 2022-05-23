use std::{
    collections::{HashSet, VecDeque},
    ops::Range, fmt::Display,
};

use typed_builder::TypedBuilder;

mod url_rep;

use crate::{models::search::url_rep::UrlRep, scrape::search::parse_search_results};

use super::{
    language::Language,
    work::{Category, Rating, Warning, WorkMetadata},
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "strum", derive(strum::EnumIter))]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Descending
    }
}

impl Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortDirection::Ascending => f.write_str("Ascending"),
            SortDirection::Descending => f.write_str("Descending"),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "strum", derive(strum::EnumIter))]
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

impl Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::BestMatch => f.write_str("Best Match"),
            SortBy::Author => f.write_str("Author"),
            SortBy::Title => f.write_str("Title"),
            SortBy::DatePosted => f.write_str("Date Posted"),
            SortBy::DateUpdated => f.write_str("Date Updated"),
            SortBy::WordCount => f.write_str("Word Count"),
            SortBy::Hits => f.write_str("Hits"),
            SortBy::Kudos => f.write_str("Kudos"),
            SortBy::Comments => f.write_str("Comments"),
            SortBy::Bookmarks => f.write_str("Bookmarks"),
        }
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
    pub completed: Option<bool>,
    pub crossover: Option<bool>,
    pub single_chapter: bool,
    pub word_count: Option<Range<usize>>,
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
    pub hits: Option<Range<usize>>,
    pub kudos: Option<Range<usize>>,
    pub comments: Option<Range<usize>>,
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
