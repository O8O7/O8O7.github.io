use dioxus::prelude::*;
use dioxus::events::keyboard_types::Key;
use web_sys::window;
use gloo_events::EventListener;
use web_sys::wasm_bindgen::JsCast;
use crate::components::controller::Controller;
use crate::components::cat::Cat;
use crate::components::bgm::BgmController;
use crate::components::bgm::play_talk_sound;
use crate::Route;

const CAT_W: f64 = 64.0;
const CAT_H: f64 = 64.0;
const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Jiji() -> Element {
    let mut stage_w = use_signal(|| 900.0_f64);
    let mut stage_h = use_signal(|| 560.0_f64);
    let mut x = use_signal(|| 418.0_f64);
    let mut y = use_signal(|| 248.0_f64);
    let mut dir = use_signal(|| true);
    let mut show_dialog = use_signal(|| false);
    let mut open_book: Signal<Option<&'static str>> = use_signal(|| None);

    let messages = ["こんにちは！", "タップしてくれてありがとう。", "Rust 勉強中だよ。", "よろしくね！"];
    let mut msg_idx    = use_signal(|| 0_usize);
    let mut char_count = use_signal(|| 0_usize);
    let mut typing     = use_signal(|| true);

    fn sync_stage(sw: &mut Signal<f64>, sh: &mut Signal<f64>, x: &mut Signal<f64>, y: &mut Signal<f64>) {
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(el) = doc.get_element_by_id("jiji-area") {
                    if let Some(el) = el.dyn_ref::<web_sys::HtmlElement>() {
                        let w = el.offset_width() as f64;
                        let h = el.offset_height() as f64;
                        // CSS が未適用 (サイズ=0) のときは更新しない
                        if w > 0.0 && h > 0.0 {
                            sw.set(w);
                            sh.set(h);
                            x.set(((w - CAT_W) / 2.0).max(0.0));
                            y.set(((h - CAT_H) / 2.0).max(0.0));
                        }
                    }
                }
            }
        }
    }

    use_future({
        let mut sw = stage_w.clone(); let mut sh = stage_h.clone();
        let mut x = x.clone();       let mut y = y.clone();
        move || async move {
            // リサイズリスナー (forget でリーク → コンポーネント寿命まで有効)
            let mut sw_r = sw.clone(); let mut sh_r = sh.clone();
            let mut x_r = x.clone();  let mut y_r = y.clone();
            EventListener::new(&window().unwrap(), "resize", move |_| {
                sync_stage(&mut sw_r, &mut sh_r, &mut x_r, &mut y_r);
            }).forget();

            // CSS 適用を待ってからレイアウト同期 (即実行だとサイズ=0 になる)
            let mut sw2 = sw.clone(); let mut sh2 = sh.clone();
            let mut x2 = x.clone();  let mut y2 = y.clone();
            gloo_timers::callback::Timeout::new(150, move || {
                sync_stage(&mut sw2, &mut sh2, &mut x2, &mut y2);
            }).forget();
        }
    });

    use_effect({
        let msg_idx    = msg_idx.clone();
        let char_count = char_count.clone();
        let typing     = typing.clone();
        move || {
            if typing() {
                let msg = messages[msg_idx()];
                if char_count() < msg.chars().count() {
                    let mut cc = char_count.clone();
                    let mut tp = typing.clone();
                    gloo_timers::callback::Timeout::new(42, move || {
                        play_talk_sound();
                        let next = cc() + 1;
                        cc.set(next);
                        if next >= msg.chars().count() { tp.set(false); }
                    }).forget();
                }
            }
        }
    });

    let on_keydown = move |evt: KeyboardEvent| {
        match evt.key() {
            Key::ArrowLeft  => { dir.set(false); x.set((x() - 20.0).max(0.0)); }
            Key::ArrowRight => { dir.set(true);  x.set((x() + 20.0).min(stage_w() - CAT_W)); }
            Key::ArrowUp    => { y.set((y() + 20.0).min(stage_h() - CAT_H)); }
            Key::ArrowDown  => { y.set((y() - 20.0).max(0.0)); }
            _ => {}
        }
    };

    let move_jiji = move |d: &'static str| {
        match d {
            "left"  => { dir.set(false); x.set((x() - 20.0).max(0.0)); }
            "right" => { dir.set(true);  x.set((x() + 20.0).min(stage_w() - CAT_W)); }
            "up"    => { y.set((y() + 20.0).min(stage_h() - CAT_H)); }
            "down"  => { y.set((y() - 20.0).max(0.0)); }
            _ => {}
        }
    };

    // Book object positions — near the bottom edge of the field
    let blog_obj_x   = (stage_w() * 0.14) as i32;
    let blog_obj_y   = 22_i32;
    let horror_obj_x = (stage_w() * 0.74) as i32;
    let horror_obj_y = 22_i32;

    // Proximity detection (cat center vs book center)
    let cat_cx = x() + CAT_W / 2.0;
    let cat_cy = y() + CAT_H / 2.0;
    let near_blog = {
        let dx = cat_cx - (blog_obj_x as f64 + 24.0);
        let dy = cat_cy - (blog_obj_y as f64 + 32.0);
        (dx * dx + dy * dy).sqrt() < 90.0
    };
    let near_horror = {
        let dx = cat_cx - (horror_obj_x as f64 + 24.0);
        let dy = cat_cy - (horror_obj_y as f64 + 32.0);
        (dx * dx + dy * dy).sqrt() < 90.0
    };

    let shown: String = if show_dialog() {
        messages[msg_idx()].chars().take(char_count()).collect()
    } else {
        String::new()
    };

    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        div {
            style: "display:flex;flex-direction:column;align-items:center;padding:1.25rem 0;",

            // ===== FIELD =====
            div {
                id: "jiji-area",
                class: "jiji-field",
                tabindex: 0,
                onkeydown: on_keydown,

                // --- Cat ---
                div {
                    style: "position:absolute;left:{x()}px;bottom:{y()}px;width:{CAT_W}px;height:{CAT_H}px;cursor:pointer;z-index:2;",
                    onclick: move |_| {
                        if !show_dialog() {
                            show_dialog.set(true);
                            msg_idx.set(0);
                            char_count.set(0);
                            typing.set(true);
                        } else {
                            show_dialog.set(false);
                        }
                    },
                    Cat { dir: dir() }
                }

                // --- Blog book object ---
                div {
                    class: "book-obj",
                    style: "left:{blog_obj_x}px;bottom:{blog_obj_y}px;",
                    onclick: move |_| open_book.set(Some("blog")),
                    if near_blog {
                        div { class: "book-badge", "📖 開く" }
                    }
                    svg {
                        width: "48", height: "64",
                        view_box: "0 0 48 64",
                        xmlns: "http://www.w3.org/2000/svg",
                        rect { x: "2", y: "1", width: "44", height: "62", rx: "2", fill: "#3a5cb8" }
                        rect { x: "2", y: "1", width: "7",  height: "62", rx: "2", fill: "#243d8a" }
                        rect { x: "44", y: "4", width: "2", height: "56", rx: "1", fill: "#ddd8b0" }
                        rect { x: "12", y: "22", width: "24", height: "2.5", rx: "1", fill: "#8090d8", opacity: "0.75" }
                        rect { x: "16", y: "28", width: "16", height: "2.5", rx: "1", fill: "#8090d8", opacity: "0.75" }
                        rect { x: "12", y: "34", width: "22", height: "2.5", rx: "1", fill: "#8090d8", opacity: "0.75" }
                        rect { x: "16", y: "40", width: "10", height: "2.5", rx: "1", fill: "#8090d8", opacity: "0.5" }
                        rect { x: "30", y: "0",  width: "8",  height: "14", fill: "#e8c040" }
                        polygon { points: "30,14 34,19 38,14", fill: "#e8c040" }
                        rect { x: "9",  y: "3",  width: "3",  height: "58", fill: "rgba(255,255,255,0.07)" }
                    }
                }

                // --- Horror tome object ---
                div {
                    class: "book-obj",
                    style: "left:{horror_obj_x}px;bottom:{horror_obj_y}px;",
                    onclick: move |_| open_book.set(Some("horror")),
                    if near_horror {
                        div { class: "book-badge", "👁 開く" }
                    }
                    svg {
                        width: "48", height: "64",
                        view_box: "0 0 48 64",
                        xmlns: "http://www.w3.org/2000/svg",
                        rect { x: "2", y: "1", width: "44", height: "62", rx: "2", fill: "#1a0a05" }
                        rect { x: "2", y: "1", width: "7",  height: "62", rx: "2", fill: "#0d0502" }
                        rect { x: "9", y: "1", width: "37", height: "62",           fill: "#1f0d06" }
                        rect { x: "11", y: "8",  width: "25", height: "47", rx: "1",
                               fill: "none", stroke: "#3a0808", stroke_width: "1" }
                        rect { x: "8",  y: "4",  width: "5", height: "5", rx: "1", fill: "#3a1a0a" }
                        rect { x: "34", y: "4",  width: "5", height: "5", rx: "1", fill: "#3a1a0a" }
                        rect { x: "8",  y: "54", width: "5", height: "5", rx: "1", fill: "#3a1a0a" }
                        rect { x: "34", y: "54", width: "5", height: "5", rx: "1", fill: "#3a1a0a" }
                        ellipse { cx: "24", cy: "32", rx: "8",   ry: "5",   fill: "none",    stroke: "#7a1010", stroke_width: "1.2" }
                        ellipse { cx: "24", cy: "32", rx: "3.5", ry: "3.5", fill: "#7a1010" }
                        circle  { cx: "24", cy: "32", r:  "1.5", fill: "#cc2020" }
                        line { x1: "15", y1: "32", x2: "13", y2: "32", stroke: "#5a0808", stroke_width: "1" }
                        line { x1: "33", y1: "32", x2: "35", y2: "32", stroke: "#5a0808", stroke_width: "1" }
                        line { x1: "24", y1: "25", x2: "24", y2: "23", stroke: "#5a0808", stroke_width: "1" }
                        line { x1: "24", y1: "39", x2: "24", y2: "41", stroke: "#5a0808", stroke_width: "1" }
                    }
                }

                // --- Nameplate ---
                div { class: "jiji-nameplate",
                    div { class: "jiji-nameplate-name", "Sakamoto Shun" }
                    div { class: "jiji-nameplate-sub",  "Rust / WASM" }
                }

                // --- Corner decorations ---
                div { class: "jiji-corner jiji-corner-tl" }
                div { class: "jiji-corner jiji-corner-tr" }
                div { class: "jiji-corner jiji-corner-bl" }
                div { class: "jiji-corner jiji-corner-br" }

                BgmController {}
            }

            // ===== CAT DIALOG (below field) =====
            if show_dialog() {
                div { class: "jiji-dialog-wrap",
                    div { class: "jiji-dialog",
                        div { class: "jiji-dialog-name", "黒猫" }
                        div { class: "jiji-dialog-text",
                            "{shown}"
                            if typing() { span { class: "jiji-cursor" } }
                        }
                        div { class: "jiji-dialog-actions",
                            if !typing() {
                                if msg_idx() < messages.len() - 1 {
                                    button {
                                        class: "jiji-dialog-next",
                                        onclick: move |_| {
                                            msg_idx.set(msg_idx() + 1);
                                            char_count.set(0);
                                            typing.set(true);
                                        },
                                        "▶  次へ"
                                    }
                                } else {
                                    span { class: "jiji-dialog-done", "— END —" }
                                }
                            }
                        }
                    }
                }
            }
        }

        Controller { move_jiji: EventHandler::new(move_jiji) }

        // ===== BOOK MODAL =====
        if let Some(book) = open_book() {
            div {
                class: "book-overlay",
                onclick: move |_| open_book.set(None),
                div {
                    class: if book == "blog" { "book-modal book-blog" } else { "book-modal book-horror" },
                    onclick: move |e| e.stop_propagation(),

                    // Left page
                    div { class: "book-page book-page-left",
                        if book == "blog" {
                            div { class: "book-blog-title", "TECH BLOG" }
                            div { class: "book-blog-author", "Sakamoto Shun" }
                            div { class: "book-ruled-lines",
                                div { class: "book-ruled-line" }
                                div { class: "book-ruled-line" }
                                div { class: "book-ruled-line" }
                                div { class: "book-ruled-line" }
                                div { class: "book-ruled-line" }
                                div { class: "book-ruled-line" }
                                div { class: "book-ruled-line" }
                                div { class: "book-ruled-line" }
                            }
                        } else {
                            div { class: "book-horror-title", "怪 談" }
                            div { class: "book-horror-sub", "眠れなくなる話" }
                            div { class: "book-horror-symbol",
                                svg {
                                    class: "book-horror-eye",
                                    view_box: "0 0 56 56",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    ellipse { cx: "28", cy: "28", rx: "22", ry: "14", fill: "none", stroke: "#5a0808", stroke_width: "2" }
                                    ellipse { cx: "28", cy: "28", rx: "10", ry: "10", fill: "#3a0808" }
                                    circle  { cx: "28", cy: "28", r:  "4",  fill: "#8b1010" }
                                    circle  { cx: "28", cy: "28", r:  "1.5", fill: "#cc2020" }
                                    line { x1: "5",  y1: "28", x2: "1",  y2: "28", stroke: "#5a0808", stroke_width: "1.5" }
                                    line { x1: "51", y1: "28", x2: "55", y2: "28", stroke: "#5a0808", stroke_width: "1.5" }
                                    line { x1: "28", y1: "10", x2: "28", y2: "6",  stroke: "#5a0808", stroke_width: "1.5" }
                                    line { x1: "28", y1: "46", x2: "28", y2: "50", stroke: "#5a0808", stroke_width: "1.5" }
                                    line { x1: "12", y1: "12", x2: "9",  y2: "9",  stroke: "#5a0808", stroke_width: "1.2" }
                                    line { x1: "44", y1: "12", x2: "47", y2: "9",  stroke: "#5a0808", stroke_width: "1.2" }
                                    line { x1: "12", y1: "44", x2: "9",  y2: "47", stroke: "#5a0808", stroke_width: "1.2" }
                                    line { x1: "44", y1: "44", x2: "47", y2: "47", stroke: "#5a0808", stroke_width: "1.2" }
                                }
                            }
                        }
                    }

                    // Right page
                    div { class: "book-page book-page-right",
                        if book == "blog" {
                            div { class: "book-right-section", "Contents" }
                            p { class: "book-right-desc",
                                "Rust や WebAssembly の学習記録。日々の気づきや実験を書き留めています。"
                            }
                            Link {
                                to: Route::BlogList,
                                class: "book-nav-btn book-nav-blog",
                                "→ 記事一覧を見る"
                            }
                        } else {
                            div { class: "book-horror-right-section", "Contents" }
                            p { class: "book-horror-right-desc",
                                "読んだら、眠れなくなる。日常のすぐそばに潜む「ありえない何か」を描く怪談集。"
                            }
                            Link {
                                to: Route::HorrorList,
                                class: "book-nav-btn book-nav-horror",
                                "→ 怪談を読む"
                            }
                        }
                    }

                    // Close button
                    button {
                        class: "book-close",
                        onclick: move |e| { e.stop_propagation(); open_book.set(None); },
                        "×"
                    }
                }
            }
        }
    }
}
