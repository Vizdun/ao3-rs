#[derive(Debug)]
/// Work Author
pub enum WorkAuthor {
    /// Anonymous
    Anonymous,
    /// orphan_acccount
    OrphanAccount,
    /// Named Author
    Named(NamedAuthor),
}

#[derive(Debug)]
/// Named Author
pub struct NamedAuthor {
    /// Pseud
    pub pseud: Option<String>,
    /// Username
    pub username: String
}

#[derive(Debug)]
/// Ratings
pub enum Rating {
    /// Not Rated
    NotRated,
    /// General Audiences
    General,
    /// Teen And Up Audiences
    Teen,
    /// Mature
    Mature,
    /// Explicit
    Explicit,
}

#[derive(Debug)]
/// Archive Warnings
pub enum Warning {
    /// Creator Chose Not To Use Archive Warnings
    CreatorChoseNotToUseArchiveWarnings,
    /// No Archive Warnings Apply
    NoArchiveWarningsApply,
    /// Graphic Depictions Of Violence
    GraphicDepictionsOfViolence,
    /// Major Character Death
    MajorCharacterDeath,
    /// Underage
    Underage,
    /// Rape/Non-Con
    RapeNonCon,
}

#[derive(Debug)]
/// Categories
pub enum Category {
    /// F/F
    FF,
    /// F/M
    FM,
    /// Gen
    Gen,
    /// M/M
    MM,
    /// Multi
    Multi,
    /// Other
    Other,
}

#[derive(Debug)]
/// A date
pub struct Date {
    /// Year
    pub year: u16,
    /// Month
    pub month: u8,
    /// Day
    pub day: u8
}

#[derive(Debug, Clone)]
/// Number of chapters of a work
pub struct ChapterCount {
    /// Number of chapters the work has currently
    pub current: u32,
    /// Number of chapters the work plans to have
    pub planned: Option<u32>,
}

#[derive(Debug)]
/// Metadata or the 'header' of a work
pub struct WorkMetadata {
    /// ID
    pub id: u32,
    /// Title
    pub title: String,
    /// Authors
    pub authors: Vec<WorkAuthor>,
    /// Summary
    pub summary: String,
    /// Rating
    pub rating: Rating,
    /// Warnings
    pub warnings: Vec<Warning>,
    /// Categories
    pub categories: Vec<Category>,
    /// Fandoms
    pub fandoms: Vec<String>,
    /// Relationships
    pub relationships: Vec<String>,
    /// Characters
    pub characters: Vec<String>,
    /// Additional tags
    pub additional_tags: Vec<String>,
    /// Language
    pub language: String,
    /// Published date
    pub published: Date,
    /// Updated date
    pub updated: Option<Date>,
    /// Word count
    pub words: u32,
    /// Number of chapters
    pub chapters: ChapterCount,
    /// Number of kudos
    pub kudos: u32,
    /// Number of hits
    pub hits: u32,
}

#[derive(Debug)]
/// Chapter of a work
pub struct Chapter {
    /// Title of the chapter
    pub title: Option<String>,
    /// Notes at the start of the chapter
    pub start_notes: Option<String>,
    /// Content/body of the chapter
    pub body: String,
    /// Notes at the end of the chapter
    pub end_notes: Option<String>,
}

#[derive(Debug)]
/// A work
pub struct Work {
    /// A struct containing information about the work's metadata
    pub metadata: WorkMetadata,
    /// Notes at the start of the work
    pub start_notes: Option<String>,
    /// Notes at the end of the work
    pub end_notes: Option<String>,
    /// Chapters of the work
    pub chapters: Vec<Chapter>,
}

impl Work {
    /// Create a work from the work's ID
    pub fn from_id(id: u32) -> Self {
        let html = reqwest::blocking::get(Work::url_from_id(id))
            .unwrap()
            .text()
            .unwrap();

        Work::parse(&html)
    }

    /// Create url to a work from the work's ID
    pub fn url_from_id(id: u32) -> String {
        format!(
            "https://archiveofourown.org/works/{}?view_full_work=true&view_adult=true",
            id
        )
    }

    /// Parse a work from HTML
    pub fn parse(html: &str) -> Self {
        crate::scrape::parse_work(html)
    }

    /// Whether the work is completed
    pub fn completed(&self) -> bool {
        match self.metadata.chapters.planned {
            Some(planned) => self.metadata.chapters.current == planned,
            None => false
        }
    }
}
