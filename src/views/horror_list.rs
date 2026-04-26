use crate::Route;
use dioxus::prelude::*;
use pulldown_cmark::{Parser, Event, Tag, HeadingLevel};
use serde::Deserialize;
use serde_json::from_str;

const HORROR_INDEX: &str = include_str!("../../assets/horrors/index.json");
const HORROR_1: &str = include_str!("../../assets/horrors/1.md");
const HORROR_CSS: Asset = asset!("/assets/styling/horror.css");

#[derive(Deserialize)]
struct HorrorMeta {
    id: i32,
    file: String,
}

pub fn get_horror_md(file: &str) -> &'static str {
    match file {
        "1.md" => HORROR_1,
        _ => "",
    }
}

fn extract_preview(md: &str) -> (String, String, String) {
    let mut title = String::new();
    let mut excerpt = String::new();
    let mut date = String::new();

    let body = if md.starts_with("---") {
        if let Some(end) = md[3..].find("---") {
            let meta = &md[3..3 + end];
            for line in meta.lines() {
                if let Some(rest) = line.trim().strip_prefix("title:") {
                    title = rest.trim().trim_matches('"').to_string();
                }
                if let Some(rest) = line.trim().strip_prefix("date:") {
                    date = rest.trim().to_string();
                }
            }
            &md[3 + end + 3..]
        } else {
            md
        }
    } else {
        md
    };

    let parser = Parser::new(body);
    let mut in_h1 = false;
    let mut found_para = false;
    for event in parser {
        match event {
            Event::Start(Tag::Heading(HeadingLevel::H1, ..)) => in_h1 = true,
            Event::End(Tag::Heading(HeadingLevel::H1, ..)) => in_h1 = false,
            Event::Text(t) if in_h1 && title.is_empty() => title = t.to_string(),
            Event::Start(Tag::Paragraph) if !found_para => found_para = true,
            Event::Text(t) if found_para && excerpt.len() < 80 => excerpt.push_str(&t),
            Event::End(Tag::Paragraph) if found_para => break,
            _ => {}
        }
    }

    if title.is_empty() {
        title = "無題".to_string();
    }
    if excerpt.is_empty() {
        excerpt = "——".to_string();
    }

    (title, excerpt, date)
}

#[component]
pub fn HorrorList() -> Element {
    let mut articles = use_signal(|| vec![]);

    use_effect({
        let mut articles = articles.clone();
        move || {
            let meta: Vec<HorrorMeta> = from_str(HORROR_INDEX).unwrap_or_default();
            let list: Vec<(i32, String, String, String)> = meta
                .into_iter()
                .map(|m| {
                    let md = get_horror_md(&m.file);
                    let (title, excerpt, date) = extract_preview(md);
                    (m.id, title, excerpt, date)
                })
                .collect();
            articles.set(list);
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: HORROR_CSS }
        div { class: "horror-page",
            div { class: "horror-list-header",
                h1 { class: "horror-list-title", "怪 談" }
                p { class: "horror-list-subtitle", "— 眠れなくなる話 —" }
                div { class: "horror-list-divider" }
            }
            div { class: "horror-grid",
                for (id, title, excerpt, date) in articles() {
                    Link {
                        to: Route::Horror { id },
                        class: "horror-card",
                        div { class: "horror-card-title", "{title}" }
                        p { class: "horror-card-excerpt", "{excerpt}……" }
                        if !date.is_empty() {
                            div { class: "horror-card-date", "{date}" }
                        }
                    }
                }
            }
        }
    }
}
