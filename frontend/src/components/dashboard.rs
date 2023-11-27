use crate::api::{
    dashboard::{close_test, create_test, get_results, get_tests},
    APIError,
};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

pub fn Dashboard(cx: Scope) -> Element {
    let nav = use_navigator(cx);

    // make a request to grab the tests to see if the person is logged in
    let tests_fut = use_future(cx, (), |_| async move { get_tests().await });
    let creation_menu = use_state(cx, || false);

    cx.render(match tests_fut.value() {
        Some(Ok(tests)) => rsx! {
            // Clicking on this button, shows a small text box to the left of the button.
            // Then there are two buttons, in the same space of the original, either cancel or
            // create.
            if *creation_menu.get() {
                rsx! {
                    button {
                        onclick: |_| creation_menu.set(false),
                        "Cancel"
                    }
                }
            }
            button {
                onclick: |_| creation_menu.set(true),
                "Create new test"
            }
            button {
                onclick: |_| tests_fut.restart(),
                "Refetch tests"
            }


            tests.iter().map(|t| {
                rsx! {
                    TestComponent { test_id: t.id, name: t.name.clone(), closed: t.closed }
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
    })
}

#[inline_props]
fn TestComponent(cx: Scope, test_id: uuid::Uuid, name: String, closed: bool) -> Element {
    let drop_down = use_state(cx, || false);
    let results = use_future(
        cx,
        test_id,
        |test_id| async move { get_results(test_id).await },
    );

    cx.render(match results.value() {
        Some(Ok(results)) => {
            rsx! {
                div {
                    button {
                        onclick: |_| drop_down.modify(|v| !v),
                        "▼"
                    }
                    "{name} : {closed}"
                }
                results.iter().map(|r| {
                    rsx! {
                        ResultComponent { test_id: r.test_id, name: r.name.clone(), level: r.level }
                    }
                })
            }
        }
        Some(Err(_)) => rsx! {
            div { "There was an issue fetching the results for this test..." }
        },
        None => rsx! { div { "Fetching the results..." } },
    })
}

#[inline_props]
fn ResultComponent(cx: Scope, test_id: uuid::Uuid, name: String, level: i32) -> Element {
    cx.render(rsx! {
        div { "{name} : {level}" }
    })
}
