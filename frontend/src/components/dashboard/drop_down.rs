use crate::api::{dashboard::get_results, APIError};
use dioxus::prelude::*;

#[inline_props]
fn TestComponent<'a>(cx: Scope, test: &'a Test) -> Element {
    let drop_down = use_state(cx, || false);
    let results = use_future(cx, &test.id, |id| async move { get_results(&id).await });

    // TODO: Implement closing a specific test

    // TODO: Add left-spaced styling to the results
    let results_rendered = match results.value() {
        Some(Ok(results)) => rsx! {
            results.iter().map(|r| {
                rsx! { ResultComponent { result:r } }
            })
        },
        Some(Err(_)) => rsx! {
                div { "There was an issue fetching the results for {test.name}..." }
        },
        None => rsx! { div { "Fetching the results..." } },
    };

    cx.render(if *drop_down.get() {
        rsx! {
            div {
                button {
                    onclick: |_| drop_down.modify(|v| !v),
                    "▲"
                }
                "{test.name}"
            }

            results_rendered
        }
    } else {
        rsx! {
           div {
               button {
                   onclick: |_| drop_down.modify(|v| !v),
                   "▼"
               }
               "{test.name}"
           }
        }
    })
}

#[inline_props]
fn ResultComponent<'a>(cx: Scope, result: &'a StudentResult) -> Element {
    cx.render(rsx! {
        div { "{result.name} : {result.level}" }
    })
}
