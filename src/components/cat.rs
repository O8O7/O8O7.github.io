use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CatProps {
    pub dir: bool,
}

#[component]
pub fn Cat(props: CatProps) -> Element {
    let flip = if props.dir { "" } else { " scale-x-[-1]" };
    let cat_class = format!("relative w-20 h-20{}", flip); // 80x80px
    rsx! {
        div {
            class: "{cat_class}",
            // しっぽ
            div { class: "absolute left-16 bottom-4 w-6 h-16 bg-gray-700 rounded-full rotate-12" }
            // 体
            div { class: "absolute left-4 bottom-4 w-12 h-12 bg-black rounded-full" }
            // 頭
            div { class: "absolute left-2 bottom-12 w-16 h-14 bg-black rounded-full border-2 border-gray-800" }
            // 耳
            div { class: "absolute left-2 bottom-24 w-5 h-7 bg-black rounded-t-full border-l-2 border-gray-800 border-t-2" }
            div { class: "absolute left-13 bottom-24 w-5 h-7 bg-black rounded-t-full border-r-2 border-gray-800 border-t-2" }
            // 顔（目・鼻・口・ほっぺ）
            div { class: "absolute left-7 bottom-20 w-2 h-2 bg-white rounded-full" } // 左目
            div { class: "absolute left-11 bottom-20 w-2 h-2 bg-white rounded-full" } // 右目
            div { class: "absolute left-9 bottom-18 w-1.5 h-1 bg-pink-300 rounded-full" } // 鼻
            div { class: "absolute left-8 bottom-17 w-4 h-2 border-b-2 border-pink-400 rounded-b-full" } // 口
            div { class: "absolute left-5 bottom-18 w-2 h-1 bg-pink-200 rounded-full opacity-70" } // 左ほっぺ
            div { class: "absolute left-13 bottom-18 w-2 h-1 bg-pink-200 rounded-full opacity-70" } // 右ほっぺ
        }
    }
} 