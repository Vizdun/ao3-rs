use scraper::ElementRef;

use crate::{
    models::work::{Chapter, ChapterCount, Work, WorkMetadata},
    scrape::{
        parse_authors, parse_id, parse_parse, parse_rating, parse_summary, parse_tags,
        parse_try_into, parse_warnings,
    },
};

pub fn parse_work(html: &str) -> Work {
    use scraper::{Html, Selector};

    let fragment = Html::parse_fragment(html);
    let fragment = fragment.root_element();

    let id_selector = Selector::parse("li.chapter.bychapter > a").unwrap();
    let title_selector = Selector::parse("h2.title").unwrap();
    let author_selector = Selector::parse("h3.byline.heading > a[rel=author]").unwrap();
    let rating_selector = Selector::parse("dd.rating a").unwrap();
    let warning_selector = Selector::parse("dd.warning a").unwrap();
    let category_selector = Selector::parse("dd.category a").unwrap();
    let fandom_selector = Selector::parse("dd.fandom a").unwrap();
    let relationship_selector = Selector::parse("dd.relationship a").unwrap();
    let character_selector = Selector::parse("dd.character a").unwrap();
    let tags_selector = Selector::parse("dd.freeform a").unwrap();
    let language_selector = Selector::parse("dd.language").unwrap();
    let published_selector = Selector::parse("dd.published").unwrap();
    let updated_selector = Selector::parse("dd.status").unwrap();
    let words_selector = Selector::parse("dd.words").unwrap();
    let chapters_selector = Selector::parse("dd.chapters").unwrap();
    let kudos_selector = Selector::parse("dd.kudos").unwrap();
    let hits_selector = Selector::parse("dd.hits").unwrap();

    let chapter_single_selector = Selector::parse("div#workskin").unwrap();
    let chapter_multiple_selector = Selector::parse("div#chapters > div.chapter").unwrap();
    let chapter_title_selector = Selector::parse("h3.title").unwrap();
    let chapter_start_notes_selector = Selector::parse("div#notes > blockquote.userstuff").unwrap();
    let chapter_end_notes_selector = Selector::parse("div.end.notes > blockquote").unwrap();
    let chapter_body_selector = Selector::parse("div.userstuff").unwrap();

    let start_notes_selector = Selector::parse(
        "div#workskin > div.preface > div.notes[role=complementary] > blockquote.userstuff",
    )
    .unwrap();
    let end_notes_selector =
        Selector::parse("div#workskin > div.preface > div.end.notes > blockquote.userstuff")
            .unwrap();

    let parse_body = |fragment: &ElementRef| -> String {
        let mut out = String::new();

        let any_selector = Selector::parse("*").unwrap();

        let mut fragments = fragment
            .select(&chapter_body_selector)
            .next()
            .unwrap()
            .select(&any_selector);

        for element in fragments.by_ref() {
            match element.value().name() {
                "p" => {
                    out.push('\n');
                    out.push_str(&element.text().collect::<String>());
                }
                "hr" => {
                    out.push_str("\n\n");
                }
                _ => {}
            }
        }

        out.trim().to_string()
    };

    let chapter_count: ChapterCount = fragment
        .select(&chapters_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap()
        .try_into()
        .unwrap();

    Work {
        metadata: WorkMetadata {
            id: {
                let scrape_title =
                    || -> Option<u32> { Some(parse_id(fragment.select(&id_selector).next()?)) };

                scrape_title().unwrap_or(0) // TODO: handle this error
            },
            title: fragment
                .select(&title_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .trim()
                .to_string(),
            authors: parse_authors(fragment.select(&author_selector)),
            summary: parse_summary(fragment),
            rating: parse_rating(fragment.select(&rating_selector).next().unwrap()),
            warnings: parse_warnings(fragment.select(&warning_selector)),
            categories: fragment
                .select(&category_selector)
                .map(|a| a.text().collect::<String>().as_str().try_into().unwrap())
                .collect(),
            fandoms: parse_tags(fragment.select(&fandom_selector)),
            relationships: parse_tags(fragment.select(&relationship_selector)),
            characters: parse_tags(fragment.select(&character_selector)),
            additional_tags: parse_tags(fragment.select(&tags_selector)),
            language: parse_try_into!(fragment.select(&language_selector)),
            published: parse_try_into!(fragment.select(&published_selector)),
            updated: fragment
                .select(&updated_selector)
                .next()
                .map(|updated_str| updated_str.text().next().unwrap().try_into().unwrap()),
            words: fragment
                .select(&words_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            chapters: chapter_count.clone(),
            kudos: parse_parse!(fragment.select(&kudos_selector)),
            hits: parse_parse!(fragment.select(&hits_selector)),
        },
        start_notes: fragment.select(&start_notes_selector).next().map(|notes| {
            notes
                .text()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join("\n")
                .trim()
                .to_string()
        }),
        end_notes: fragment.select(&end_notes_selector).next().map(|notes| {
            notes
                .text()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join("\n")
                .trim()
                .to_string()
        }),
        chapters: match chapter_count.current {
            1 => fragment
                .select(&chapter_single_selector)
                .map(|p| Chapter {
                    title: Some(
                        fragment
                            .select(&title_selector)
                            .next()
                            .unwrap()
                            .text()
                            .next()
                            .unwrap()
                            .trim()
                            .to_string(),
                    ),
                    start_notes: p.select(&chapter_start_notes_selector).next().map(|notes| {
                        notes
                            .text()
                            .map(|a| a.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                            .trim()
                            .to_string()
                    }),
                    body: parse_body(&p),
                    end_notes: p.select(&chapter_end_notes_selector).next().map(|notes| {
                        notes
                            .text()
                            .map(|a| a.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                            .trim()
                            .to_string()
                    }),
                })
                .collect::<Vec<Chapter>>(),
            _ => fragment
                .select(&chapter_multiple_selector)
                .map(|p| Chapter {
                    title: p
                        .select(&chapter_title_selector)
                        .next()
                        .unwrap()
                        .text()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                        .splitn(2, ':')
                        .collect::<Vec<&str>>()
                        .get(1)
                        .map(|s| s.trim().to_string()),
                    start_notes: p.select(&chapter_start_notes_selector).next().map(|notes| {
                        notes
                            .text()
                            .map(|a| a.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                            .trim()
                            .to_string()
                    }),
                    body: parse_body(&p),
                    end_notes: p.select(&chapter_end_notes_selector).next().map(|notes| {
                        notes
                            .text()
                            .map(|a| a.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                            .trim()
                            .to_string()
                    }),
                })
                .collect::<Vec<Chapter>>(),
        },
    }
}
