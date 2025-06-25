use dioxus::prelude::*;
use dioxus::events::keyboard_types::Key;
use web_sys::window;
use gloo_events::EventListener;
use crate::components::controller::Controller;
use crate::components::cat::Cat;

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

    use_future(move || async move {
        let w = window().unwrap().inner_width().unwrap().as_f64().unwrap_or(1000.0);
        let h = window().unwrap().inner_height().unwrap().as_f64().unwrap_or(600.0) - 100.0;
        stage_width.set(w.max(CAT_WIDTH + 20.0));
        stage_height.set(h.max(CAT_HEIGHT + 20.0));
        x.set((x() as f64).min(stage_width() - CAT_WIDTH).max(0.0));
        y.set((y() as f64).min(stage_height() - CAT_HEIGHT).max(0.0));
    });

    use_effect(move || {
        let mut stage_width = stage_width.clone();
        let mut stage_height = stage_height.clone();
        let mut x = x.clone();
        let mut y = y.clone();
        let listener = EventListener::new(&window().unwrap(), "resize", move |_event| {
            let w = window().unwrap().inner_width().unwrap().as_f64().unwrap_or(1000.0);
            let h = window().unwrap().inner_height().unwrap().as_f64().unwrap_or(600.0) - 100.0;
            stage_width.set(w.max(CAT_WIDTH + 20.0));
            stage_height.set(h.max(CAT_HEIGHT + 20.0));
            x.set((x() as f64).min(stage_width() - CAT_WIDTH).max(0.0));
            y.set((y() as f64).min(stage_height() - CAT_HEIGHT).max(0.0));
        });
        (move || drop(listener))()
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

    rsx! {
        div {
            style: {format!("position: relative; width: {}px; height: {}px; background: #f8f8ff; border: 2px solid #444; margin: 2rem auto; outline: none; border-radius: 1.5rem;", stage_width(), stage_height())},
            tabindex: 0,
            onkeydown: on_keydown,
            div {
                style: {format!("position: absolute; left: {}px; bottom: {}px; width: {}px; height: {}px; cursor: pointer;", x(), y(), CAT_WIDTH, CAT_HEIGHT)},
                onclick: move |_| show_intro.set(!show_intro()),
                Cat { dir: dir() }
            }
            if show_intro() {
                div {
                    style: {format!("position: absolute; left: {}px; bottom: {}px; background: #fff; color: #222; border: 1px solid #333; border-radius: 8px; padding: 0.5rem 1rem; min-width: 180px; z-index: 10; box-shadow: 2px 2px 8px #888; font-family: 'serif';", x(), y() + CAT_HEIGHT + 10.0)},
                    b { "猫です" }
                    br {}
                    span { "はじめまして！" }
                }
            }
            Controller { move_jiji: EventHandler::new(move_jiji) }
        }
    }
} 