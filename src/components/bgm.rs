use dioxus::prelude::*;
use web_sys::{AudioContext, OscillatorNode};
use web_sys::js_sys::Reflect;
use gloo_timers::callback::Interval;

static mut BGM_LOOP: Option<Interval> = None;

pub fn play_bgm() {
    let ctx = AudioContext::new().unwrap();
    // 3音のアルペジオをループ再生する例
    let notes = [261.63, 329.63, 392.00]; // C, E, G
    for (i, &freq) in notes.iter().enumerate() {
        let osc = ctx.create_oscillator().unwrap();
        Reflect::set(osc.as_ref(), &"type".into(), &"sine".into()).unwrap();
        osc.frequency().set_value(freq as f32);
        osc.connect_with_audio_node(&ctx.destination()).unwrap();
        let t = ctx.current_time() + i as f64 * 0.3;
        osc.start_with_when(t).unwrap();
        osc.stop_with_when(t + 0.25).unwrap();
    }
    // ループ再生はsetTimeout/Intervalやwasm-timerで再帰的に呼ぶのが本格的ですが、
    // ここでは1回だけ鳴らす簡易例です。
}

pub fn start_bgm_loop() {
    unsafe {
        if BGM_LOOP.is_none() {
            play_bgm();
            BGM_LOOP = Some(Interval::new(900, || play_bgm()));
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
                // 再生中は音符が揺れるアニメーション
                span { class: "animate-bounce", "♪" }
            } else {
                span { "♪" }
            }
        }
    }
} 