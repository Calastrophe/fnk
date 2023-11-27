use crate::api::{
    dashboard::{close_test, create_test, get_results, get_tests},
    APIError,
};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

pub fn Dashboard(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let tests_fut = use_future(cx, (), |_| async move { get_tests().await });

    cx.render(match tests_fut.value() {
        Some(Ok(tests)) => rsx! {

            CreationMenu {}

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

fn CreationMenu(cx: Scope) -> Element {
    let visible = use_state(cx, || false);
    let resp_text = use_state(cx, || None::<String>);

    let on_submit = move |evt: FormEvent| {
        to_owned![resp_text];

        cx.spawn(async move {
            let resp = create_test(evt.values["name"][0].as_str()).await;

            match resp {
                Err(e) => match e {
                    APIError::Validation(validation_errs) => {
                        resp_text.set(Some(validation_errs.join(",")))
                    }
                    _ => resp_text.set(Some(e.to_string())),
                },
                Ok(_) => resp_text.set(Some("Successfully created".to_string())),
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
            resp_text.set(None);
        });
    };

    cx.render(if *visible.get() {
        rsx! {
            div {
                form {
                    input { name: "name", },
                    button {
                        onclick: move |_| visible.set(false),
                        "Cancel"
                    }
                    button { "Submit" }
                }
            }
        }
    } else {
        rsx! {
            button {
                onclick: move |_| visible.set(true),
                "Create a new test"
            }
        }
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
