use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CatProps {
    pub dir: bool,
}

#[component]
pub fn Cat(props: CatProps) -> Element {
    let flip = if props.dir { "" } else { "transform: scaleX(-1);" };
    rsx! {
        div {
            style: "width:64px;height:64px;{flip}",
            svg {
                width: "64",
                height: "64",
                view_box: "0 0 64 64",
                xmlns: "http://www.w3.org/2000/svg",

                // shadow
                ellipse { cx: "30", cy: "62", rx: "12", ry: "2.5", fill: "rgba(0,0,0,0.22)" }

                // tail (draw before body)
                path {
                    d: "M46 50 Q62 35 54 18",
                    stroke: "#16162a",
                    stroke_width: "5",
                    fill: "none",
                    stroke_linecap: "round"
                }

                // body
                ellipse { cx: "30", cy: "48", rx: "16", ry: "12", fill: "#16162a" }

                // head
                circle { cx: "30", cy: "27", r: "14", fill: "#16162a" }

                // left ear outer
                polygon { points: "17,18 12,4 26,14", fill: "#16162a" }
                // right ear outer
                polygon { points: "43,18 49,4 35,14", fill: "#16162a" }
                // left ear inner (pink)
                polygon { points: "17,16 14,8 25,13", fill: "#e8889a" }
                // right ear inner (pink)
                polygon { points: "43,16 47,8 36,13", fill: "#e8889a" }

                // left eye
                ellipse { cx: "24", cy: "27", rx: "4.5", ry: "5", fill: "#6ae585" }
                // right eye
                ellipse { cx: "37", cy: "27", rx: "4.5", ry: "5", fill: "#6ae585" }
                // left pupil
                ellipse { cx: "24", cy: "27", rx: "2.5", ry: "4.2", fill: "#0d0d1a" }
                // right pupil
                ellipse { cx: "37", cy: "27", rx: "2.5", ry: "4.2", fill: "#0d0d1a" }
                // eye shine left
                circle { cx: "25.5", cy: "25", r: "1.2", fill: "white" }
                // eye shine right
                circle { cx: "38.5", cy: "25", r: "1.2", fill: "white" }

                // nose
                path { d: "M28 33 L30 35.5 L32 33 Z", fill: "#f4a8bc" }
                // mouth
                path {
                    d: "M28 35.5 Q30 37.5 32 35.5",
                    stroke: "#9a7080",
                    stroke_width: "0.9",
                    fill: "none",
                    stroke_linecap: "round"
                }

                // whiskers left
                line { x1: "6",  y1: "30", x2: "21", y2: "32", stroke: "#555", stroke_width: "0.7" }
                line { x1: "6",  y1: "34", x2: "21", y2: "34", stroke: "#555", stroke_width: "0.7" }
                // whiskers right
                line { x1: "39", y1: "32", x2: "54", y2: "30", stroke: "#555", stroke_width: "0.7" }
                line { x1: "39", y1: "34", x2: "54", y2: "34", stroke: "#555", stroke_width: "0.7" }

                // front paws
                ellipse { cx: "22", cy: "58", rx: "7",   ry: "4.5", fill: "#16162a" }
                ellipse { cx: "38", cy: "58", rx: "7",   ry: "4.5", fill: "#16162a" }
                // paw toe lines
                line { x1: "18", y1: "57", x2: "18", y2: "61", stroke: "#2a2a4a", stroke_width: "1" }
                line { x1: "22", y1: "58", x2: "22", y2: "62", stroke: "#2a2a4a", stroke_width: "1" }
                line { x1: "26", y1: "57", x2: "26", y2: "61", stroke: "#2a2a4a", stroke_width: "1" }
                line { x1: "34", y1: "57", x2: "34", y2: "61", stroke: "#2a2a4a", stroke_width: "1" }
                line { x1: "38", y1: "58", x2: "38", y2: "62", stroke: "#2a2a4a", stroke_width: "1" }
                line { x1: "42", y1: "57", x2: "42", y2: "61", stroke: "#2a2a4a", stroke_width: "1" }
            }
        }
    }
}
