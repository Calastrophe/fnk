use dioxus::prelude::*;

#[inline_props]
pub fn Test(cx: Scope, id: String) -> Element {
    render! {
        div { "todo" }
    }
}
