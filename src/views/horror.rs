use crate::Route;
use dioxus::prelude::*;
use pulldown_cmark::{Parser, Options, html, Event, Tag, HeadingLevel};
use crate::views::horror_list::get_horror_md;

const HORROR_CSS: Asset = asset!("/assets/styling/horror.css");

fn parse_horror(md: &str) -> (String, String, String) {
    let mut title = String::new();
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

    let parser = Parser::new_ext(body, Options::all());
    let mut events = Vec::new();
    let mut skip_h1 = false;
    for event in parser {
        match &event {
            Event::Start(Tag::Heading(HeadingLevel::H1, ..)) => skip_h1 = true,
            Event::End(Tag::Heading(HeadingLevel::H1, ..)) => skip_h1 = false,
            _ if skip_h1 => {}
            _ => events.push(event),
        }
    }
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, events.into_iter());

    (title, date, html_buf)
}

#[component]
pub fn Horror(id: i32) -> Element {
    let mut content = use_signal(|| String::new());
    let mut title = use_signal(|| String::new());
    let mut date = use_signal(|| String::new());

    use_effect({
        let mut content = content.clone();
        let mut title = title.clone();
        let mut date = date.clone();
        move || {
            let md = get_horror_md(&format!("{}.md", id));
            if md.is_empty() {
                content.set("<p>記事が見つかりません</p>".to_string());
                return;
            }
            let (t, d, html) = parse_horror(md);
            title.set(t);
            date.set(d);
            content.set(html);
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: HORROR_CSS }
        div { class: "horror-page",
            div { class: "horror-detail",
                div { class: "horror-detail-meta",
                    h1 { class: "horror-detail-title", "{title()}" }
                    if !date().is_empty() {
                        p { class: "horror-detail-date", "{date()}" }
                    }
                    div { class: "horror-detail-divider" }
                }
                div {
                    class: "horror-body",
                    dangerous_inner_html: content()
                }
                Link {
                    to: Route::HorrorList,
                    class: "horror-back",
                    "← 怪談一覧へ"
                }
            }
        }
    }
}
