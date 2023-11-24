use dioxus::prelude::*;

pub fn Dashboard(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "todo" }
    })
}
