use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ControllerProps {
    pub move_jiji: EventHandler<&'static str>,
}

#[component]
pub fn Controller(props: ControllerProps) -> Element {
    rsx! {
        div {
            class: "fixed left-4 bottom-4 flex flex-col items-center z-50",
            style: "pointer-events: auto;",
            div {
                class: "flex justify-center mb-2",
                button {
                    class: "w-16 h-16 rounded-full bg-blue-600 text-white text-2xl font-bold shadow-lg border-2 border-blue-900 active:bg-blue-800 select-none",
                    onclick: move |_| props.move_jiji.call("up"),
                    "↑"
                }
            }
            div {
                class: "flex justify-center",
                button {
                    class: "w-16 h-16 rounded-full bg-blue-600 text-white text-2xl font-bold shadow-lg border-2 border-blue-900 mr-8 active:bg-blue-800 select-none",
                    onclick: move |_| props.move_jiji.call("left"),
                    "←"
                }
                button {
                    class: "w-16 h-16 rounded-full bg-blue-600 text-white text-2xl font-bold shadow-lg border-2 border-blue-900 active:bg-blue-800 select-none",
                    onclick: move |_| props.move_jiji.call("down"),
                    "↓"
                }
                button {
                    class: "w-16 h-16 rounded-full bg-blue-600 text-white text-2xl font-bold shadow-lg border-2 border-blue-900 ml-8 active:bg-blue-800 select-none",
                    onclick: move |_| props.move_jiji.call("right"),
                    "→"
                }
            }
        }
    }
} 