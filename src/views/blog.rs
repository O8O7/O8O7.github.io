use crate::Route;
use dioxus::prelude::*;
use pulldown_cmark::{Parser, Options, html, Event, Tag, HeadingLevel};

const BLOG_1: &str = include_str!("../../assets/blogs/1.md");
const BLOG_2: &str = include_str!("../../assets/blogs/2.md");
const BLOG_3: &str = include_str!("../../assets/blogs/3.md");
const BLOG_4: &str = include_str!("../../assets/blogs/4.md");
const BLOG_5: &str = include_str!("../../assets/blogs/5.md");
const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

fn get_blog_md(id: i32) -> &'static str {
    match id {
        1 => BLOG_1,
        2 => BLOG_2,
        3 => BLOG_3,
        4 => BLOG_4,
        5 => BLOG_5,
        _ => "",
    }
}

fn parse_meta_and_body(md: &str) -> (Option<String>, Option<String>, String) {
    let mut title = None;
    let mut date = None;
    let mut body = md;
    if md.starts_with("---") {
        if let Some(end) = md[3..].find("---") {
            let meta = &md[3..3+end];
            body = &md[3+end+3..];
            for line in meta.lines() {
                if let Some(rest) = line.trim().strip_prefix("title:") {
                    title = Some(rest.trim().trim_matches('"').to_string());
                }
                if let Some(rest) = line.trim().strip_prefix("date:") {
                    date = Some(rest.trim().to_string());
                }
            }
        }
    }
    let parser = Parser::new_ext(body, Options::all());
    let mut filtered_events = Vec::new();
    let mut skip_h1 = false;
    for event in parser {
        match &event {
            Event::Start(Tag::Heading(HeadingLevel::H1, ..)) => skip_h1 = true,
            Event::End(Tag::Heading(HeadingLevel::H1, ..)) => skip_h1 = false,
            _ if skip_h1 => {},
            _ => filtered_events.push(event),
        }
    }
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, filtered_events.into_iter());
    (title, date, html_buf)
}

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
///
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Blog(id: i32) -> Element {
    let mut html_content = use_signal(|| String::from("読み込み中..."));
    let mut meta = use_signal(|| (None, None));

    use_effect({
        let mut html_content = html_content.clone();
        let mut meta = meta.clone();
        move || {
            let md = get_blog_md(id);
            if md.is_empty() {
                html_content.set("記事が見つかりません".to_string());
                meta.set((None, None));
                return;
            }
            let (title, date, filtered_html) = parse_meta_and_body(md);
            meta.set((title, date));
            html_content.set(filtered_html);
        }
    });

    let (title, date) = meta();

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }
        div {
            style: "min-height: 100vh; background: #0f1116; color: #fff; padding: 2rem 0;",
            div {
                class: "prose max-w-2xl mx-auto p-6 rounded-xl shadow-lg relative",
                style: "background: rgba(24,26,32,0.96); color: #fff; border: 1px solid #333;",
                if let Some(title) = &title {
                    h1 { style: "color: #ffb86b; font-size: 2.2rem; font-weight: bold; margin-bottom: 0.5rem; text-align: left;", "{title}" }
                }
                if let Some(date) = &date {
                    div { style: "position: absolute; top: 1.5rem; right: 2rem; color: #ffd580; font-size: 1rem;", "{date}" }
                }
                div { dangerous_inner_html: html_content() }
            }
            style { r#"
                .prose h1 {{ color: #ffb86b; font-size: 2.2rem; font-weight: bold; margin-bottom: 1.2rem; }}
                .prose h2 {{ color: #ffd580; font-size: 1.5rem; font-weight: bold; margin-top: 2rem; margin-bottom: 1rem; }}
                .prose h3 {{ color: #ffe9b3; font-size: 1.2rem; font-weight: bold; margin-top: 1.5rem; margin-bottom: 0.8rem; }}
                .prose p, .prose li {{ color: #fff; }}
                .prose code {{ background: #23262f; color: #ffb86b; padding: 2px 6px; border-radius: 4px; }}
                .prose pre {{ background: #23262f; color: #fff; border-radius: 8px; padding: 1em; }}
                .prose strong {{ color: #ffd580; }}
                .prose ul {{ list-style: disc; margin-left: 1.5em; }}
                .prose ol {{ list-style: decimal; margin-left: 1.5em; }}
                .prose a {{ color: #91a4d2; text-decoration: underline; }}
            "# }
        }
    }
}
