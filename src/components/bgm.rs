use dioxus::prelude::*;
use web_sys::{AudioContext, OscillatorNode};
use web_sys::js_sys::Reflect;
use gloo_timers::callback::Interval;

static mut BGM_LOOP: Option<Interval> = None;

pub fn play_bgm() {
    let ctx = AudioContext::new().unwrap();
    let notes = [196.00, 220.00, 246.94, 261.63, 293.66]; // G3, A3, B3, C4, D4
    for (i, &freq) in notes.iter().enumerate() {
        let osc = ctx.create_oscillator().unwrap();
        Reflect::set(osc.as_ref(), &"type".into(), &"triangle".into()).unwrap();
        osc.frequency().set_value(freq as f32);
        let gain = ctx.create_gain().unwrap();
        gain.gain().set_value(0.13);
        osc.connect_with_audio_node(&gain).unwrap();
        gain.connect_with_audio_node(&ctx.destination()).unwrap();
        let t = ctx.current_time() + i as f64 * 0.45;
        osc.start_with_when(t).unwrap();
        osc.stop_with_when(t + 0.4).unwrap();
    }
}

pub fn play_talk_sound() {
    let ctx = AudioContext::new().unwrap();
    let osc = ctx.create_oscillator().unwrap();
    Reflect::set(osc.as_ref(), &"type".into(), &"square".into()).unwrap();
    osc.frequency().set_value(800.0);
    let gain = ctx.create_gain().unwrap();
    gain.gain().set_value(0.08);
    osc.connect_with_audio_node(&gain).unwrap();
    gain.connect_with_audio_node(&ctx.destination()).unwrap();
    let t = ctx.current_time();
    osc.start_with_when(t).unwrap();
    osc.stop_with_when(t + 0.06).unwrap();
}

pub fn start_bgm_loop() {
    unsafe {
        if BGM_LOOP.is_none() {
            play_bgm();
            BGM_LOOP = Some(Interval::new(2300, || play_bgm()));
        }
    }
}

pub fn stop_bgm_loop() {
    unsafe {
        if let Some(loop_handle) = BGM_LOOP.take() {
            drop(loop_handle);
        }
    }
}

#[component]
pub fn BgmController() -> Element {
    let mut playing = use_signal(|| false);
    let toggle_bgm = move |_| {
        if !playing() {
            start_bgm_loop();
            playing.set(true);
        } else {
            stop_bgm_loop();
            playing.set(false);
        }
    };
    rsx! {
        button {
            class: "fixed top-4 right-4 z-50 w-14 h-14 rounded-full bg-indigo-600 text-white shadow-lg flex items-center justify-center text-3xl hover:bg-indigo-700 active:bg-indigo-800 transition-all",
            onclick: toggle_bgm,
            title: if playing() { "BGM停止" } else { "BGM再生" },
            aria_label: "BGM再生/停止",
            if playing() {
                span { class: "animate-bounce", "♪" }
            } else {
                span { "♪" }
            }
        }
    }
} 