use dioxus::prelude::*;

pub fn Canvas(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "todo" }
    })
}
