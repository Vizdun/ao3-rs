use crate::{
    models::work::WorkMetadata,
    scrape::{
        parse_authors, parse_id, parse_parse, parse_rating, parse_summary, parse_tags,
        parse_try_into, parse_warnings,
    },
};

pub fn parse_search_results(html: &str) -> Vec<WorkMetadata> {
    use scraper::{Html, Selector};

    let fragment = Html::parse_fragment(html);

    // let id_selector = Selector::parse("li.chapter.bychapter > a").unwrap();
    // let title_selector = Selector::parse("h2.title").unwrap();
    // let author_selector = Selector::parse("h3.byline.heading > a[rel=author]").unwrap();
    // let summary_selector = Selector::parse("blockquote.userstuff").unwrap();
    // let rating_selector = Selector::parse("dd.rating a").unwrap();
    // let warning_selector = Selector::parse("dd.warning a").unwrap();
    // let category_selector = Selector::parse("dd.category a").unwrap();
    // let fandom_selector = Selector::parse("dd.fandom a").unwrap();
    // let relationship_selector = Selector::parse("dd.relationship a").unwrap();
    // let character_selector = Selector::parse("dd.character a").unwrap();
    // let tags_selector = Selector::parse("dd.freeform a").unwrap();
    // let language_selector = Selector::parse("dd.language").unwrap();
    // let published_selector = Selector::parse("dd.published").unwrap();
    // let updated_selector = Selector::parse("dd.status").unwrap();
    // let words_selector = Selector::parse("dd.words").unwrap();
    // let chapters_selector = Selector::parse("dd.chapters").unwrap();
    // let kudos_selector = Selector::parse("dd.kudos").unwrap();
    // let hits_selector = Selector::parse("dd.hits").unwrap();

    // let chapter_single_selector = Selector::parse("div#workskin").unwrap();
    // let chapter_multiple_selector = Selector::parse("div#chapters > div.chapter").unwrap();
    // let chapter_title_selector = Selector::parse("h3.title").unwrap();
    // let chapter_start_notes_selector = Selector::parse("div#notes > blockquote.userstuff").unwrap();
    // let chapter_end_notes_selector = Selector::parse("div.end.notes > blockquote").unwrap();
    // let chapter_body_selector = Selector::parse("div.userstuff").unwrap();

    // let start_notes_selector = Selector::parse(
    //     "div#workskin > div.preface > div.notes[role=complementary] > blockquote.userstuff",
    // )
    // .unwrap();
    // let end_notes_selector =
    //     Selector::parse("div#workskin > div.preface > div.end.notes > blockquote.userstuff")
    //         .unwrap();

    let result_selector = Selector::parse("li.work.blurb.group").unwrap();

    let title_selector = Selector::parse("h4.heading > a").unwrap();
    let author_selector = Selector::parse("h4.heading > a[rel=author]").unwrap();

    let rating_selector = Selector::parse("span.rating > span").unwrap();
    let categories_selector = Selector::parse("span.category > span").unwrap();
    let warning_selector = Selector::parse("li.warnings > strong > a").unwrap();

    let fandom_selector = Selector::parse("h5.fandoms.heading > a").unwrap();
    let relationship_selector = Selector::parse("li.relationships > a").unwrap();
    let character_selector = Selector::parse("li.characters > a").unwrap();
    let tags_selector = Selector::parse("li.freeforms > a").unwrap();

    let language_selector = Selector::parse("dd.language").unwrap();
    let updated_selector = Selector::parse("p.datetime").unwrap();
    let word_count_selector = Selector::parse("dd.words").unwrap();
    let chapters_selector = Selector::parse("dd.chapters").unwrap();
    let kudos_selector = Selector::parse("dd.kudos > a").unwrap();
    let hits_selector = Selector::parse("dd.hits").unwrap();

    fragment
        .select(&result_selector)
        .map(|e| WorkMetadata {
            id: parse_id(e.select(&title_selector).next().unwrap()),
            title: e
                .select(&title_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .trim()
                .to_string(),
            authors: parse_authors(e.select(&author_selector)),
            summary: parse_summary(e),
            rating: parse_rating(e.select(&rating_selector).next().unwrap()),
            warnings: parse_warnings(e.select(&warning_selector)),
            categories: e
                .select(&categories_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.trim().try_into())
                .filter(|r| r.is_ok())
                .map(|r| r.unwrap())
                .collect(),
            fandoms: parse_tags(e.select(&fandom_selector)),
            relationships: parse_tags(e.select(&relationship_selector)),
            characters: parse_tags(e.select(&character_selector)),
            additional_tags: parse_tags(e.select(&tags_selector)),
            language: parse_try_into!(e.select(&language_selector)),
            published: parse_try_into!(e.select(&updated_selector)), // NOTE: search results don't have published dates
            updated: Some(parse_try_into!(e.select(&updated_selector))),
            words: e
                .select(&word_count_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .chars()
                .filter(|c| *c != ',')
                .collect::<String>()
                .parse()
                .unwrap(),
            chapters: e
                .select(&chapters_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .as_str()
                .try_into()
                .unwrap(),
            kudos: parse_parse!(e.select(&kudos_selector)),
            hits: parse_parse!(e.select(&hits_selector)),
        })
        .collect()
}
