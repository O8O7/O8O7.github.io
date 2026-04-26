use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ControllerProps {
    pub move_jiji: EventHandler<&'static str>,
}

#[component]
pub fn Controller(props: ControllerProps) -> Element {
    let mut up_timer    = use_signal(|| None);
    let mut left_timer  = use_signal(|| None);
    let mut down_timer  = use_signal(|| None);
    let mut right_timer = use_signal(|| None);

    let start_repeat = move |dir: &'static str, timer_signal: &mut Signal<Option<i32>>| {
        props.move_jiji.call(dir);
        let handler = props.move_jiji.clone();
        let id = gloo_timers::callback::Interval::new(80, move || handler.call(dir));
        timer_signal.set(Some(Box::leak(Box::new(id)) as *mut _ as i32));
    };
    let stop_repeat = move |timer_signal: &mut Signal<Option<i32>>| {
        if let Some(id) = timer_signal() {
            unsafe { let _ = Box::from_raw(id as *mut gloo_timers::callback::Interval); }
            timer_signal.set(None);
        }
    };

    rsx! {
        div {
            class: "dpad",
            // Up
            button {
                class: "dpad-btn dpad-up",
                onclick:     move |_| props.move_jiji.call("up"),
                onmousedown: move |_| start_repeat("up", &mut up_timer),
                onmouseup:   move |_| stop_repeat(&mut up_timer),
                onmouseleave:move |_| stop_repeat(&mut up_timer),
                ontouchstart:move |e| { e.prevent_default(); start_repeat("up", &mut up_timer); },
                ontouchend:  move |_| stop_repeat(&mut up_timer),
                "▲"
            }
            // Left
            button {
                class: "dpad-btn dpad-left",
                onclick:     move |_| props.move_jiji.call("left"),
                onmousedown: move |_| start_repeat("left", &mut left_timer),
                onmouseup:   move |_| stop_repeat(&mut left_timer),
                onmouseleave:move |_| stop_repeat(&mut left_timer),
                ontouchstart:move |e| { e.prevent_default(); start_repeat("left", &mut left_timer); },
                ontouchend:  move |_| stop_repeat(&mut left_timer),
                "◀"
            }
            // Center decoration
            div { class: "dpad-center",
                div { class: "dpad-center-dot" }
            }
            // Down
            button {
                class: "dpad-btn dpad-down",
                onclick:     move |_| props.move_jiji.call("down"),
                onmousedown: move |_| start_repeat("down", &mut down_timer),
                onmouseup:   move |_| stop_repeat(&mut down_timer),
                onmouseleave:move |_| stop_repeat(&mut down_timer),
                ontouchstart:move |e| { e.prevent_default(); start_repeat("down", &mut down_timer); },
                ontouchend:  move |_| stop_repeat(&mut down_timer),
                "▼"
            }
            // Right
            button {
                class: "dpad-btn dpad-right",
                onclick:     move |_| props.move_jiji.call("right"),
                onmousedown: move |_| start_repeat("right", &mut right_timer),
                onmouseup:   move |_| stop_repeat(&mut right_timer),
                onmouseleave:move |_| stop_repeat(&mut right_timer),
                ontouchstart:move |e| { e.prevent_default(); start_repeat("right", &mut right_timer); },
                ontouchend:  move |_| stop_repeat(&mut right_timer),
                "▶"
            }
        }
    }
}
