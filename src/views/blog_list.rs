use crate::Route;
use dioxus::prelude::*;
use pulldown_cmark::{Parser, Event, Tag, HeadingLevel};
use serde::Deserialize;
use serde_json::from_str;

const BLOG_INDEX: &str = include_str!("../../assets/blogs/index.json");
const BLOG_1: &str = include_str!("../../assets/blogs/1.md");
const BLOG_2: &str = include_str!("../../assets/blogs/2.md");
const BLOG_3: &str = include_str!("../../assets/blogs/3.md");
const BLOG_4: &str = include_str!("../../assets/blogs/4.md");
const BLOG_5: &str = include_str!("../../assets/blogs/5.md");

#[derive(Deserialize)]
struct BlogMeta {
    id: i32,
    file: String,
}

fn get_blog_md(file: &str) -> &'static str {
    match file {
        "1.md" => BLOG_1,
        "2.md" => BLOG_2,
        "3.md" => BLOG_3,
        "4.md" => BLOG_4,
        "5.md" => BLOG_5,
        _ => "",
    }
}

#[component]
pub fn BlogList() -> Element {
    let mut articles = use_signal(|| vec![]);
    let mut error_msg = use_signal(|| None);

    use_effect({
        let mut articles = articles.clone();
        let mut error_msg = error_msg.clone();
        move || {
            let meta: Result<Vec<BlogMeta>, _> = from_str(BLOG_INDEX);
            match meta {
                Ok(meta) => {
                    let mut list = vec![];
                    for m in meta {
                        let md = get_blog_md(&m.file);
                        let parser = Parser::new(md);
                        let mut title = String::new();
                        let mut excerpt = String::new();
                        let mut in_h1 = false;
                        let mut found_excerpt = false;
                        for event in parser {
                            match event {
                                Event::Start(Tag::Heading(HeadingLevel::H1, ..)) => in_h1 = true,
                                Event::End(Tag::Heading(HeadingLevel::H1, ..)) => in_h1 = false,
                                Event::Text(text) if in_h1 => title.push_str(&text),
                                Event::Start(Tag::Paragraph) if !found_excerpt => found_excerpt = true,
                                Event::Text(text) if found_excerpt && excerpt.len() < 60 => excerpt.push_str(&text),
                                Event::End(Tag::Paragraph) if found_excerpt => break,
                                _ => {}
                            }
                        }
                        if title.is_empty() { title = "No Title".to_string(); }
                        if excerpt.is_empty() { excerpt = "（抜粋なし）".to_string(); }
                        list.push((m.id, title, excerpt));
                    }
                    articles.set(list);
                }
                Err(e) => {
                    error_msg.set(Some(format!("記事リストの読み込みに失敗しました: {}", e)));
                    articles.set(vec![]);
                }
            }
        }
    });

    rsx! {
        div { class: "min-h-screen py-10", style: "background: #0f1116; color: #fff;",
            div { class: "max-w-2xl mx-auto",
                h1 { class: "text-3xl font-bold mb-8 text-center", style: "color: #ffb86b; letter-spacing: 0.1em;", "Rustブログ" }
                if let Some(msg) = error_msg() {
                    div { class: "text-red-400 text-center mb-6 font-bold", "{msg}" }
                }
                div { class: "grid gap-6",
                    for (id, title, excerpt) in articles() {
                        Link {
                            to: Route::Blog { id },
                            class: "block rounded-xl shadow-md transition p-6 border border-[#333] hover:shadow-lg hover:bg-[#23262f] cursor-pointer bg-[#181a20]",
                            h2 { class: "text-2xl font-bold mb-2", style: "color: #ffb86b; letter-spacing: 0.05em;", "{title}" }
                            p { class: "text-gray-200 text-base mt-1", "{excerpt}..." }
                        }
                    }
                }
            }
        }
    }
} 