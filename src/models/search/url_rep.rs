use std::{collections::HashSet, ops::Range};

use crate::models::{
    language::Language,
    work::{Category, Rating, Warning},
};

use super::{SortBy, SortDirection};

impl UrlRep for Option<bool> {
    fn url(&self) -> String {
        match self {
            Some(true) => "T",
            Some(false) => "F",
            None => "",
        }
        .to_string()
    }
}

impl<T: UrlRep> UrlRep for Option<T> {
    fn url(&self) -> String {
        match self {
            Some(t) => t.url(),
            None => String::new(),
        }
    }
}

impl UrlRep for Range<usize> {
    fn url(&self) -> String {
        format!("{}-{}", self.start, self.end)
    }
}

impl UrlRep for Language {
    fn url(&self) -> String {
        let s: &str = self.into();

        s.to_string()
    }
}

impl UrlRep for HashSet<String> {
    fn url(&self) -> String {
        self.iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl UrlRep for Rating {
    fn url(&self) -> String {
        match self {
            Rating::NotRated => "9",
            Rating::General => "10",
            Rating::Teen => "11",
            Rating::Mature => "12",
            Rating::Explicit => "13",
        }
        .to_string()
    }
}

impl UrlRep for Warning {
    fn url(&self) -> String {
        match self {
            Warning::CreatorChoseNotToUseArchiveWarnings => "14",
            Warning::GraphicDepictionsOfViolence => "17",
            Warning::MajorCharacterDeath => "18",
            Warning::NoArchiveWarningsApply => "16",
            Warning::RapeNonCon => "19",
            Warning::Underage => "20",
        }
        .to_string()
    }
}

impl UrlRep for Category {
    fn url(&self) -> String {
        match self {
            Category::FF => "116",
            Category::FM => "22",
            Category::Gen => "21",
            Category::MM => "23",
            Category::Multi => "2246",
            Category::Other => "24",
        }
        .to_string()
    }
}

impl UrlRep for SortBy {
    fn url(&self) -> String {
        match self {
            SortBy::BestMatch => "_score",
            SortBy::Author => "authors_to_sort_on",
            SortBy::Title => "title_to_sort_on",
            SortBy::DatePosted => "created_at",
            SortBy::DateUpdated => "revised_at",
            SortBy::WordCount => "word_count",
            SortBy::Hits => "hits",
            SortBy::Kudos => "kudos_count",
            SortBy::Comments => "comments_count",
            SortBy::Bookmarks => "bookmarks_count",
        }
        .to_string()
    }
}

impl UrlRep for SortDirection {
    fn url(&self) -> String {
        match self {
            SortDirection::Ascending => "asc",
            SortDirection::Descending => "desc",
        }
        .to_string()
    }
}

pub trait UrlRep {
    fn url(&self) -> String;
}
