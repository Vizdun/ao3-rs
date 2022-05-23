use super::language::Language;

#[derive(Debug, Clone)]
/// Work Author
pub enum WorkAuthor {
    /// Anonymous
    Anonymous,
    /// orphan_acccount
    OrphanAccount,
    /// Named Author
    Named(NamedAuthor),
}

#[derive(Debug, Clone)]
/// Named Author
pub struct NamedAuthor {
    /// Pseud
    pub pseud: Option<String>,
    /// Username
    pub username: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "strum", derive(strum::EnumIter))]
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

impl TryFrom<&str> for Rating {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Not Rated" => Ok(Rating::NotRated),
            "General Audiences" => Ok(Rating::General),
            "Teen And Up Audiences" => Ok(Rating::Teen),
            "Mature" => Ok(Rating::Mature),
            "Explicit" => Ok(Rating::Explicit),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "strum", derive(strum::EnumIter))]
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

impl TryFrom<&str> for Warning {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Creator Chose Not To Use Archive Warnings" => {
                Ok(Warning::CreatorChoseNotToUseArchiveWarnings)
            }
            "No Archive Warnings Apply" => Ok(Warning::NoArchiveWarningsApply),
            "Graphic Depictions Of Violence" => Ok(Warning::GraphicDepictionsOfViolence),
            "Major Character Death" => Ok(Warning::MajorCharacterDeath),
            "Underage" => Ok(Warning::Underage),
            "Rape/Non-Con" => Ok(Warning::RapeNonCon),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "strum", derive(strum::EnumIter))]
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

impl TryFrom<&str> for Category {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "F/F" => Ok(Category::FF),
            "F/M" => Ok(Category::FM),
            "Gen" => Ok(Category::Gen),
            "M/M" => Ok(Category::MM),
            "Multi" => Ok(Category::Multi),
            "Other" => Ok(Category::Other),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
/// A date
pub struct Date {
    /// Year
    pub year: u16,
    /// Month
    pub month: u8,
    /// Day
    pub day: u8,
}

impl TryFrom<&str> for Date {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.len() {
            10 => Ok(Date {
                year: value[0..4].parse().or(Err(()))?,
                month: value[5..7].parse().or(Err(()))?,
                day: value[8..10].parse().or(Err(()))?,
            }),
            11 => Ok(Date {
                year: value[7..11].parse().or(Err(()))?,
                month: match &value[3..6] {
                    "Jan" => 1,
                    "Feb" => 2,
                    "Mar" => 3,
                    "Apr" => 4,
                    "May" => 5,
                    "Jun" => 6,
                    "Jul" => 7,
                    "Aug" => 8,
                    "Sep" => 9,
                    "Oct" => 10,
                    "Nov" => 11,
                    "Dec" => 12,
                    _ => Err(())?,
                },
                day: value[0..2].parse().or(Err(()))?,
            }),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
/// Number of chapters of a work
pub struct ChapterCount {
    /// Number of chapters the work has currently
    pub current: u32,
    /// Number of chapters the work plans to have
    pub planned: Option<u32>,
}

impl TryFrom<&str> for ChapterCount {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chapters_arr = value.splitn(2, '/').collect::<Vec<&str>>();

        Ok(ChapterCount {
            current: chapters_arr[0].parse().unwrap(),
            planned: if chapters_arr[1] != "?" {
                Some(chapters_arr[1].parse().unwrap())
            } else {
                None
            },
        })
    }
}

#[derive(Debug, Clone)]
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
    pub language: Language,
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
            None => false,
        }
    }
}
