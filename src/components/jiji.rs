use dioxus::prelude::*;
use dioxus::events::keyboard_types::Key;
use web_sys::window;
use gloo_events::EventListener;
use web_sys::wasm_bindgen::JsCast;
use crate::components::controller::Controller;
use crate::components::cat::Cat;
use crate::components::bgm::BgmController;
use crate::components::bgm::play_talk_sound;

const CAT_WIDTH: f64 = 80.0;
const CAT_HEIGHT: f64 = 80.0;

#[component]
pub fn Jiji() -> Element {
    let mut stage_width = use_signal(|| 1000.0);
    let mut stage_height = use_signal(|| 600.0);
    let mut x = use_signal(|| 460.0); // (1000-80)/2
    let mut y = use_signal(|| 260.0); // (600-80)/2
    let mut dir = use_signal(|| true);
    let mut show_intro = use_signal(|| false);

    // RPG風メッセージ配列
    let messages = [
        "こんにちは！",
        "タップしてくれてありがとう。",
        "Rust勉強中です。",
        "よろしくね！"
    ];
    let mut msg_idx = use_signal(|| 0);
    let mut char_count = use_signal(|| 0);
    let mut typing = use_signal(|| true);

    use_future(move || async move {
        let w = window().unwrap().inner_width().unwrap().as_f64().unwrap_or(1000.0);
        let h = window().unwrap().inner_height().unwrap().as_f64().unwrap_or(600.0) - 100.0;
        stage_width.set(w.max(CAT_WIDTH + 20.0));
        stage_height.set(h.max(CAT_HEIGHT + 20.0));
        x.set((x() as f64).min(stage_width() - CAT_WIDTH).max(0.0));
        y.set((y() as f64).min(stage_height() - CAT_HEIGHT).max(0.0));
    });

    // エリアの幅・高さをDOMから取得してstage_width, stage_heightにセット
    fn update_stage_size(stage_width: &mut Signal<f64>, stage_height: &mut Signal<f64>, x: &mut Signal<f64>, y: &mut Signal<f64>) {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(area) = document.get_element_by_id("jiji-area") {
                    if let Some(el) = area.dyn_ref::<web_sys::HtmlElement>() {
                        let width = el.offset_width() as f64;
                        let height = el.offset_height() as f64;
                        stage_width.set(width);
                        stage_height.set(height);
                        x.set((width - CAT_WIDTH) / 2.0);
                        y.set((height - CAT_HEIGHT) / 2.0);
                    }
                }
            }
        }
    }

    use_future({
        let mut stage_width = stage_width.clone();
        let mut stage_height = stage_height.clone();
        let mut x = x.clone();
        let mut y = y.clone();
        move || async move {
            update_stage_size(&mut stage_width, &mut stage_height, &mut x, &mut y);
        }
    });

    use_effect({
        let mut stage_width = stage_width.clone();
        let mut stage_height = stage_height.clone();
        let mut x = x.clone();
        let mut y = y.clone();
        move || {
            let mut stage_width = stage_width.clone();
            let mut stage_height = stage_height.clone();
            let mut x = x.clone();
            let mut y = y.clone();
            let listener = EventListener::new(&window().unwrap(), "resize", move |_event| {
                update_stage_size(&mut stage_width, &mut stage_height, &mut x, &mut y);
            });
            (move || drop(listener))()
        }
    });

    // 1文字ずつ表示＋ピコ音
    use_effect({
        let msg_idx = msg_idx.clone();
        let char_count = char_count.clone();
        let typing = typing.clone();
        move || {
            if typing() {
                let msg = messages[msg_idx()];
                if char_count() < msg.chars().count() {
                    let mut char_count = char_count.clone();
                    let mut typing = typing.clone();
                    gloo_timers::callback::Timeout::new(40, move || {
                        play_talk_sound();
                        char_count.set(char_count() + 1);
                        if char_count() + 1 >= msg.chars().count() {
                            typing.set(false);
                        }
                    }).forget();
                }
            }
        }
    });

    let on_keydown = move |evt: KeyboardEvent| {
        match evt.key() {
            Key::ArrowLeft => {
                dir.set(false);
                let new_x = (x() - 20.0_f64).max(0.0_f64);
                x.set(new_x);
            }
            Key::ArrowRight => {
                dir.set(true);
                let new_x = (x() + 20.0_f64).min(stage_width() - CAT_WIDTH);
                x.set(new_x);
            }
            Key::ArrowUp => {
                let new_y = (y() + 20.0_f64).min(stage_height() - CAT_HEIGHT);
                y.set(new_y);
            }
            Key::ArrowDown => {
                let new_y = (y() - 20.0_f64).max(0.0_f64);
                y.set(new_y);
            }
            _ => {}
        }
    };

    // move_jijiはEventHandlerでラップ
    let move_jiji = move |direction: &'static str| {
        match direction {
            "left" => {
                dir.set(false);
                let new_x = (x() - 20.0_f64).max(0.0_f64);
                x.set(new_x);
            }
            "right" => {
                dir.set(true);
                let new_x = (x() + 20.0_f64).min(stage_width() - CAT_WIDTH);
                x.set(new_x);
            }
            "up" => {
                let new_y = (y() + 20.0_f64).min(stage_height() - CAT_HEIGHT);
                y.set(new_y);
            }
            "down" => {
                let new_y = (y() - 20.0_f64).max(0.0_f64);
                y.set(new_y);
            }
            _ => {}
        }
    };

    // 吹き出し用メッセージを事前に用意
    let (msg, shown) = if show_intro() {
        let msg = messages[msg_idx()];
        let shown = msg.chars().take(char_count()).collect::<String>();
        (Some(msg), Some(shown))
    } else {
        (None, None)
    };

    rsx! {
        div {
            id: "jiji-area",
            style: {
                format!(
                    "
                    position: relative;
                    width: calc(100vw - 32px);
                    height: min(90vh, 700px);
                    max-width: 100vw;
                    max-height: 100vh;
                    background: repeating-linear-gradient(90deg, #d6e5b1 0 8%, #c8d6a3 8% 16%),
                                repeating-linear-gradient(0deg, #d6e5b1 0 8%, #c8d6a3 8% 16%);
                    background-size: 60px 60px;
                    border: 12px solid #8b5c2a;
                    border-radius: 2rem;
                    box-shadow: 0 0 0 8px #e7d7b1, 0 4px 24px #0004;
                    margin: 2rem auto;
                    box-sizing: border-box;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    overflow: hidden;
                    "
                )
            },
            tabindex: 0,
            onkeydown: on_keydown,
            div {
                style: {format!("position: absolute; left: {}px; bottom: {}px; width: {}px; height: {}px; cursor: pointer; z-index: 2;", x(), y(), CAT_WIDTH, CAT_HEIGHT)},
                onclick: move |_| show_intro.set(!show_intro()),
                Cat { dir: dir() }
            }
            if show_intro() {
                div {
                    style: {format!("position: absolute; left: {}px; bottom: {}px; background: #fff; color: #222; border: 1px solid #333; border-radius: 8px; padding: 0.5rem 1rem; min-width: 180px; z-index: 10; box-shadow: 2px 2px 8px #888; font-family: 'serif';", x(), y() + CAT_HEIGHT + 10.0)},
                    b { "ジジ（猫）" }
                    br {}
                    span { dangerous_inner_html: shown.as_deref().unwrap_or("") }
                    if !typing() {
                        if msg_idx() < messages.len() - 1 {
                            button {
                                class: "ml-4 px-2 py-1 rounded bg-indigo-200 text-indigo-900 text-sm font-bold border border-indigo-400 hover:bg-indigo-300 transition-all",
                                onclick: move |_| {
                                    msg_idx.set(msg_idx() + 1);
                                    char_count.set(0);
                                    typing.set(true);
                                },
                                "▶ 次へ"
                            }
                        } else {
                            span { class: "ml-4 text-gray-500 text-xs", "（おわり）" }
                        }
                    }
                }
            }
            // 木の柱風の四隅装飾
            div { style: "position: absolute; left:0; top:0; width:32px; height:32px; background:#8b5c2a; border-radius: 0 0 2rem 0; z-index:1;" }
            div { style: "position: absolute; right:0; top:0; width:32px; height:32px; background:#8b5c2a; border-radius: 0 0 0 2rem; z-index:1;" }
            div { style: "position: absolute; left:0; bottom:0; width:32px; height:32px; background:#8b5c2a; border-radius: 0 2rem 0 0; z-index:1;" }
            div { style: "position: absolute; right:0; bottom:0; width:32px; height:32px; background:#8b5c2a; border-radius: 2rem 0 0 0; z-index:1;" }
            Controller { move_jiji: EventHandler::new(move_jiji) }
            BgmController {}
        }
    }
} 