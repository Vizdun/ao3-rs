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