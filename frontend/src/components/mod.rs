pub mod canvas;
pub mod dashboard;
pub mod forms;
pub mod test;
use dioxus::prelude::*;

#[inline_props]
pub fn NotFound(cx: Scope, route: Vec<String>) -> Element {
    let strobe_css = "
        @keyframes strobe {
            0%   { background-color: red; }
            25%  { background-color: blue; }
            50%  { background-color: green; }
            75%  { background-color: yellow; }
            100% { background-color: red; }
        }
        .strobe-effect {
            animation: strobe 7.1s linear infinite;
        }
    ";

    cx.render(rsx! {
        style { strobe_css }
        div{
            class: "strobe-effect",
            style: "height: 100vh; width: 100vw; display: flex; justify-content: center; align-items: center;",
            div {
                class: "grid grid-cols-1-lg gap-1 text-center",
                p {
                    class: "text-white-900 text-white text-9xl",
                    "404 :("
                },
                p {
                    class: "text-white-900 text-white text-9xl",
                    "NOT FOUND"
                }
            }

        }
    })
}
