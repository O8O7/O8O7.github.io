use dioxus::prelude::*;
use web_sys::{AudioContext, GainNode};
use web_sys::js_sys::Reflect;
use gloo_timers::callback::Interval;

static mut BGM_CTX:   Option<AudioContext> = None;
static mut MASTER:    Option<GainNode>     = None; // master volume bus
static mut TALK_CTX:  Option<AudioContext> = None;
static mut BGM_LOOP:  Option<Interval>     = None;
static mut BGM_VOLUME: f32 = 0.25;

fn init_bgm() -> AudioContext {
    unsafe {
        if BGM_CTX.is_none() {
            let ctx = AudioContext::new().unwrap();
            let master = ctx.create_gain().unwrap();
            master.gain().set_value(BGM_VOLUME);
            master.connect_with_audio_node(&ctx.destination()).unwrap();
            MASTER  = Some(master);
            BGM_CTX = Some(ctx);
        }
        BGM_CTX.as_ref().unwrap().clone()
    }
}

fn acquire_talk_ctx() -> AudioContext {
    unsafe {
        if TALK_CTX.is_none() {
            TALK_CTX = Some(AudioContext::new().unwrap());
        }
        TALK_CTX.as_ref().unwrap().clone()
    }
}

// Immediately updates currently-playing audio via the master GainNode.
pub fn set_bgm_volume(v: f32) {
    unsafe {
        BGM_VOLUME = v;
        if let Some(ref master) = MASTER {
            master.gain().set_value(v);
        }
    }
}

// Exponential attack/release — no click artifacts.
// vol is 0–1 relative to master gain.
fn note(ctx: &AudioContext, freq: f64, t: f64, dur: f64, vol: f32, wave: &str) {
    let osc = ctx.create_oscillator().unwrap();
    Reflect::set(osc.as_ref(), &"type".into(), &wave.into()).unwrap();
    osc.frequency().set_value(freq as f32);

    let gn = ctx.create_gain().unwrap();
    let g  = gn.gain();
    let atk = 0.015_f64.min(dur * 0.15);
    let rel = 0.08_f64.min(dur * 0.35);
    // 0.001 ≈ -60 dB; exponential ramps cannot start/end at exactly 0
    let _ = g.set_value_at_time(0.001, t);
    let _ = g.exponential_ramp_to_value_at_time(vol, t + atk);
    let _ = g.set_value_at_time(vol * 0.78, t + dur - rel);
    let _ = g.exponential_ramp_to_value_at_time(0.001, t + dur);

    osc.connect_with_audio_node(&gn).unwrap();
    // Route through master so the slider controls live audio
    unsafe {
        if let Some(ref master) = MASTER {
            gn.connect_with_audio_node(master).unwrap();
        } else {
            gn.connect_with_audio_node(&ctx.destination()).unwrap();
        }
    }
    osc.start_with_when(t).unwrap();
    osc.stop_with_when(t + dur + 0.05).unwrap();
}

pub fn play_bgm() {
    let ctx = init_bgm();
    let t0  = ctx.current_time() + 0.08;

    // === Melody (triangle) ===
    // 4-bar phrase: C → Am → F → G  @  BPM=120
    // e=staccato eighth(0.22s)  q=quarter(0.47s)
    let melody: &[(f64, f64, f64)] = &[
        // Bar 1 – C
        (329.63, 0.00, 0.22), (392.00, 0.25, 0.22), (440.00, 0.50, 0.22),
        (392.00, 0.75, 0.22), (329.63, 1.00, 0.22), (392.00, 1.25, 0.22),
        (440.00, 1.50, 0.47),
        // Bar 2 – Am
        (440.00, 2.00, 0.22), (493.88, 2.25, 0.22), (523.25, 2.50, 0.22),
        (493.88, 2.75, 0.22), (440.00, 3.00, 0.22), (392.00, 3.25, 0.22),
        (329.63, 3.50, 0.47),
        // Bar 3 – F
        (349.23, 4.00, 0.22), (392.00, 4.25, 0.22), (440.00, 4.50, 0.22),
        (392.00, 4.75, 0.22), (349.23, 5.00, 0.22), (329.63, 5.25, 0.22),
        (293.66, 5.50, 0.47),
        // Bar 4 – G (descending cadence)
        (392.00, 6.00, 0.22), (349.23, 6.25, 0.22), (329.63, 6.50, 0.22),
        (293.66, 6.75, 0.22), (261.63, 7.00, 0.22), (293.66, 7.25, 0.22),
        (196.00, 7.50, 0.47),
    ];
    for &(freq, offset, dur) in melody {
        note(&ctx, freq, t0 + offset, dur, 0.70, "triangle");
    }

    // === Bass (sine) ===
    let bass: &[(f64, f64, f64)] = &[
        (130.81, 0.0, 0.85), (196.00, 1.0, 0.85),
        (110.00, 2.0, 0.85), (164.81, 3.0, 0.85),
        (174.61, 4.0, 0.85), (130.81, 5.0, 0.85),
        (196.00, 6.0, 0.85), (146.83, 7.0, 0.85),
    ];
    for &(freq, offset, dur) in bass {
        note(&ctx, freq, t0 + offset, dur, 0.50, "sine");
    }
}

pub fn play_talk_sound() {
    let ctx = acquire_talk_ctx();
    let t = ctx.current_time() + 0.001;
    let osc = ctx.create_oscillator().unwrap();
    Reflect::set(osc.as_ref(), &"type".into(), &"square".into()).unwrap();
    osc.frequency().set_value(720.0);
    let gn = ctx.create_gain().unwrap();
    let g  = gn.gain();
    let _ = g.set_value_at_time(0.001, t);
    let _ = g.linear_ramp_to_value_at_time(0.06, t + 0.005);
    let _ = g.exponential_ramp_to_value_at_time(0.001, t + 0.032);
    osc.connect_with_audio_node(&gn).unwrap();
    gn.connect_with_audio_node(&ctx.destination()).unwrap();
    osc.start_with_when(t).unwrap();
    osc.stop_with_when(t + 0.045).unwrap();
}

pub fn start_bgm_loop() {
    unsafe {
        if BGM_LOOP.is_none() {
            play_bgm();
            BGM_LOOP = Some(Interval::new(8200, || play_bgm()));
        }
    }
}

pub fn stop_bgm_loop() {
    unsafe {
        if let Some(h) = BGM_LOOP.take() { drop(h); }
    }
}

#[component]
pub fn BgmController() -> Element {
    let mut playing = use_signal(|| false);
    let mut vol     = use_signal(|| unsafe { BGM_VOLUME });

    let toggle = move |_| {
        if !playing() {
            start_bgm_loop();
            playing.set(true);
        } else {
            stop_bgm_loop();
            playing.set(false);
        }
    };

    rsx! {
        div {
            class: "fixed top-4 right-4 z-50 flex flex-col items-end gap-2",

            // Volume panel — appears while BGM is playing
            if playing() {
                div {
                    class: "flex items-center gap-2 rounded-lg px-3 py-2 border border-gray-700 shadow-lg",
                    style: "background:rgba(17,17,30,0.92);backdrop-filter:blur(4px);",
                    span { class: "text-gray-400 text-sm select-none", "🔈" }
                    input {
                        r#type: "range",
                        class: "w-24 accent-indigo-400",
                        style: "height:4px;",
                        min: "0", max: "100", step: "5",
                        value: "{(vol() * 100.0) as i32}",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<f32>() {
                                let v = (v / 100.0).clamp(0.0, 1.0);
                                set_bgm_volume(v); // instant effect via master GainNode
                                vol.set(v);
                            }
                        }
                    }
                    span { class: "text-gray-400 text-sm select-none", "🔊" }
                }
            }

            button {
                class: "w-12 h-12 rounded-full bg-indigo-600 text-white shadow-lg flex items-center justify-center text-2xl hover:bg-indigo-700 active:bg-indigo-800 transition-all",
                onclick: toggle,
                title: if playing() { "BGM停止" } else { "BGM再生" },
                aria_label: "BGM再生/停止",
                if playing() { span { class: "animate-bounce", "♪" } } else { span { "♪" } }
            }
        }
    }
}
