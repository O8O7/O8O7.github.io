use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ControllerProps {
    pub move_jiji: EventHandler<&'static str>,
}

#[component]
pub fn Controller(props: ControllerProps) -> Element {
    // 各方向ごとにタイマーIDを保持
    let mut up_timer = use_signal(|| None);
    let mut left_timer = use_signal(|| None);
    let mut down_timer = use_signal(|| None);
    let mut right_timer = use_signal(|| None);

    // 長押し開始
    let start_repeat = move |dir: &'static str, timer_signal: &mut Signal<Option<i32>>| {
        props.move_jiji.call(dir);
        let handler = props.move_jiji.clone();
        let id = gloo_timers::callback::Interval::new(80, move || {
            handler.call(dir);
        });
        timer_signal.set(Some(Box::leak(Box::new(id)) as *mut _ as i32));
    };
    // 長押し終了
    let stop_repeat = move |timer_signal: &mut Signal<Option<i32>>| {
        if let Some(id) = timer_signal() {
            unsafe { Box::from_raw(id as *mut gloo_timers::callback::Interval); }
            timer_signal.set(None);
        }
    };

    rsx! {
        div {
            class: "fixed left-4 bottom-4 flex flex-col items-center z-50",
            style: "pointer-events: auto;",
            div {
                class: "flex justify-center mb-2",
                button {
                    class: "w-16 h-16 rounded-full bg-blue-600 text-white text-2xl font-bold shadow-lg border-2 border-blue-900 active:bg-blue-800 select-none",
                    onclick: move |_| props.move_jiji.call("up"),
                    onmousedown: move |_| start_repeat("up", &mut up_timer),
                    onmouseup: move |_| stop_repeat(&mut up_timer),
                    onmouseleave: move |_| stop_repeat(&mut up_timer),
                    ontouchstart: move |evt| { evt.prevent_default(); start_repeat("up", &mut up_timer); },
                    ontouchend: move |_| stop_repeat(&mut up_timer),
                    ondblclick: move |evt| evt.prevent_default(),
                    "↑"
                }
            }
            div {
                class: "flex justify-center",
                button {
                    class: "w-16 h-16 rounded-full bg-blue-600 text-white text-2xl font-bold shadow-lg border-2 border-blue-900 mr-8 active:bg-blue-800 select-none",
                    onclick: move |_| props.move_jiji.call("left"),
                    onmousedown: move |_| start_repeat("left", &mut left_timer),
                    onmouseup: move |_| stop_repeat(&mut left_timer),
                    onmouseleave: move |_| stop_repeat(&mut left_timer),
                    ontouchstart: move |evt| { evt.prevent_default(); start_repeat("left", &mut left_timer); },
                    ontouchend: move |_| stop_repeat(&mut left_timer),
                    ondblclick: move |evt| evt.prevent_default(),
                    "←"
                }
                button {
                    class: "w-16 h-16 rounded-full bg-blue-600 text-white text-2xl font-bold shadow-lg border-2 border-blue-900 active:bg-blue-800 select-none",
                    onclick: move |_| props.move_jiji.call("down"),
                    onmousedown: move |_| start_repeat("down", &mut down_timer),
                    onmouseup: move |_| stop_repeat(&mut down_timer),
                    onmouseleave: move |_| stop_repeat(&mut down_timer),
                    ontouchstart: move |evt| { evt.prevent_default(); start_repeat("down", &mut down_timer); },
                    ontouchend: move |_| stop_repeat(&mut down_timer),
                    ondblclick: move |evt| evt.prevent_default(),
                    "↓"
                }
                button {
                    class: "w-16 h-16 rounded-full bg-blue-600 text-white text-2xl font-bold shadow-lg border-2 border-blue-900 ml-8 active:bg-blue-800 select-none",
                    onclick: move |_| props.move_jiji.call("right"),
                    onmousedown: move |_| start_repeat("right", &mut right_timer),
                    onmouseup: move |_| stop_repeat(&mut right_timer),
                    onmouseleave: move |_| stop_repeat(&mut right_timer),
                    ontouchstart: move |evt| { evt.prevent_default(); start_repeat("right", &mut right_timer); },
                    ontouchend: move |_| stop_repeat(&mut right_timer),
                    ondblclick: move |evt| evt.prevent_default(),
                    "→"
                }
            }
        }
    }
} 