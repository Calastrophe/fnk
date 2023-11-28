use crate::api::{
    dashboard::{get_results, get_tests, StudentResult, Test},
    APIError,
};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use navbar::NavBar;

mod navbar;

pub fn Dashboard(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let tests_fut = use_future(cx, (), |_| async move { get_tests().await });

    let tests_rendered = match tests_fut.value() {
        Some(Ok(tests)) => rsx! {
            tests.iter().map(|t| {
                rsx! {
                    TestComponent { test: t }
                }
            })
        },
        Some(Err(e)) => match e {
            APIError::Authorization(_) => {
                nav.push(crate::Route::Login {});
                rsx! { div { "Redirecting..." } }
            }
            _ => rsx! {
                div { "There was an issue when fetching your tests..." }
            },
        },
        None => rsx! { div { "Fetching the tests..." } },
    };

    cx.render(rsx! {
        NavBar {
            onrefresh: move |_| tests_fut.restart(),
        }

        tests_rendered
    })
}

#[inline_props]
fn TestComponent<'a>(cx: Scope, test: &'a Test) -> Element {
    let drop_down = use_state(cx, || false);
    let results = use_future(cx, &test.id, |id| async move { get_results(&id).await });

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
