use scraper::ElementRef;

use crate::enums::*;
use crate::structs::*;

pub fn parse_work(html: &str) -> Work {
    use scraper::{Html, Selector};

    let fragment = Html::parse_fragment(&html);

    let id_selector = Selector::parse("li.chapter.bychapter > a").unwrap();
    let title_selector = Selector::parse("h2.title").unwrap();
    let author_selector = Selector::parse("h3.byline.heading > a[rel=author]").unwrap();
    let summary_selector = Selector::parse("blockquote.userstuff").unwrap();
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

        while let Some(element) = fragments.next() {
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

    let chapter_count: ChapterCount = {
        let chapters_arr = fragment
            .select(&chapters_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .splitn(2, "/")
            .collect::<Vec<&str>>();

        ChapterCount {
            current: chapters_arr[0].parse().unwrap(),
            planned: if chapters_arr[1] != "?" {
                Some(chapters_arr[1].parse().unwrap())
            } else {
                None
            },
        }
    };

    Work {
        metadata: WorkMetadata {
            id: fragment
                .select(&id_selector)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()[7..]
                .splitn(2, "/")
                .next()
                .unwrap()
                .parse()
                .unwrap(),
            title: fragment
                .select(&title_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .trim()
                .to_string(),
            authors: fragment
                .select(&author_selector)
                .map(|a| a.text().collect())
                .collect(),
            summary: fragment
                .select(&summary_selector)
                .next()
                .unwrap()
                .text()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join("\n")
                .trim()
                .to_string(),
            rating: match fragment
                .select(&rating_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
            {
                "Not Rated" => Rating::NotRated,
                "General Audiences" => Rating::General,
                "Teen And Up Audiences" => Rating::Teen,
                "Mature" => Rating::Mature,
                "Explicit" => Rating::Explicit,
                _ => {
                    panic!["invalid rating"];
                }
            },
            warnings: fragment
                .select(&warning_selector)
                .map(|a| match a.text().collect::<String>().as_str() {
                    "Creator Chose Not To Use Archive Warnings" => {
                        Warning::CreatorChoseNotToUseArchiveWarnings
                    }
                    "No Archive Warnings Apply" => Warning::NoArchiveWarningsApply,
                    "Graphic Depictions Of Violence" => Warning::GraphicDepictionsOfViolence,
                    "Major Character Death" => Warning::MajorCharacterDeath,
                    "Underage" => Warning::Underage,
                    "Rape/Non-Con" => Warning::RapeNonCon,
                    _ => {
                        panic!["invalid warning"];
                    }
                })
                .collect(),
            categories: fragment
                .select(&category_selector)
                .map(|a| match a.text().collect::<String>().as_str() {
                    "F/F" => Category::FF,
                    "F/M" => Category::FM,
                    "Gen" => Category::Gen,
                    "M/M" => Category::MM,
                    "Multi" => Category::Multi,
                    "Other" => Category::Other,
                    _ => {
                        panic!["invalid warning"];
                    }
                })
                .collect(),
            fandoms: fragment
                .select(&fandom_selector)
                .map(|a| a.text().collect())
                .collect(),
            relationships: fragment
                .select(&relationship_selector)
                .map(|a| a.text().collect())
                .collect(),
            characters: fragment
                .select(&character_selector)
                .map(|a| a.text().collect())
                .collect(),
            additional_tags: fragment
                .select(&tags_selector)
                .map(|a| a.text().collect())
                .collect(),
            language: fragment
                .select(&language_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .trim()
                .to_string(),
            published: {
                let published_str = fragment
                    .select(&published_selector)
                    .next()
                    .unwrap()
                    .text()
                    .next()
                    .unwrap();

                Date {
                    year: published_str[0..4].parse().unwrap(),
                    month: published_str[5..7].parse().unwrap(),
                    day: published_str[8..10].parse().unwrap(),
                }
            },
            updated: {
                match fragment.select(&updated_selector).next() {
                    Some(updated_str) => {
                        let updated_str = updated_str.text().next().unwrap();
                        Some(Date {
                            year: updated_str[0..4].parse().unwrap(),
                            month: updated_str[5..7].parse().unwrap(),
                            day: updated_str[8..10].parse().unwrap(),
                        })
                    }
                    None => None,
                }
            },
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
            kudos: fragment
                .select(&kudos_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
            hits: fragment
                .select(&hits_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
        },
        start_notes: match fragment.select(&start_notes_selector).next() {
            Some(notes) => Some(
                notes
                    .text()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
                    .trim()
                    .to_string(),
            ),
            None => None,
        },
        end_notes: match fragment.select(&end_notes_selector).next() {
            Some(notes) => Some(
                notes
                    .text()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
                    .trim()
                    .to_string(),
            ),
            None => None,
        },
        chapters: match chapter_count.current {
            1 => fragment
                .select(&chapter_single_selector)
                .map(|p| Chapter {
                    title: fragment
                        .select(&title_selector)
                        .next()
                        .unwrap()
                        .text()
                        .next()
                        .unwrap()
                        .trim()
                        .to_string(),
                    start_notes: match p.select(&chapter_start_notes_selector).next() {
                        Some(notes) => Some(
                            notes
                                .text()
                                .map(|a| a.to_string())
                                .collect::<Vec<String>>()
                                .join("\n")
                                .trim()
                                .to_string(),
                        ),
                        None => None,
                    },
                    body: parse_body(&p),
                    end_notes: match p.select(&chapter_end_notes_selector).next() {
                        Some(notes) => Some(
                            notes
                                .text()
                                .map(|a| a.to_string())
                                .collect::<Vec<String>>()
                                .join("\n")
                                .trim()
                                .to_string(),
                        ),
                        None => None,
                    },
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
                        .splitn(2, ":")
                        .collect::<Vec<&str>>()[1]
                        .trim()
                        .to_string(),
                    start_notes: match p.select(&chapter_start_notes_selector).next() {
                        Some(notes) => Some(
                            notes
                                .text()
                                .map(|a| a.to_string())
                                .collect::<Vec<String>>()
                                .join("\n")
                                .trim()
                                .to_string(),
                        ),
                        None => None,
                    },
                    body: parse_body(&p),
                    end_notes: match p.select(&chapter_end_notes_selector).next() {
                        Some(notes) => Some(
                            notes
                                .text()
                                .map(|a| a.to_string())
                                .collect::<Vec<String>>()
                                .join("\n")
                                .trim()
                                .to_string(),
                        ),
                        None => None,
                    },
                })
                .collect::<Vec<Chapter>>(),
        },
    }
}
