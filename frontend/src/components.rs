pub mod dashboard;
pub mod drawing;
pub mod forms;
pub mod test;
use dioxus::prelude::*;

#[inline_props]
pub fn NotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        div { "not found" }
    }
}
