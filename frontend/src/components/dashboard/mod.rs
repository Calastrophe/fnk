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

        div { class: "py-14 overflow-x-auto",
            table { class: "min-w-full bg-white font-[sans-serif]",
                thead { class: "bg-gray-100 whitespace-nowrap",
                    tr {
                        th { class: "px-6 py-3 text-left text-sm font-semibold text-black",
                            "Name"
                        }
                        th { class: "px-6 py-3 text-left text-sm font-semibold text-black",
                            "Status"
                        }
                        th { class: "px-6 py-3 text-left text-sm font-semibold text-black",
                            "Link"
                        }
                    }
                }
                tbody { class: "whitespace-nowrap divide-y divide-gray-200",
                    tests_rendered
                }
            }
        }
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

    let is_empty = match results.value() {
        Some(Ok(v)) => v.is_empty(),
        _ => true,
    };

    cx.render(rsx! {
           tr { class: "hover:bg-blue-50 pl-6 w-8",
               td { class: "px-6 py-3 text-sm cursor-pointer",
                    onclick: |_| drop_down.modify(|v| !v),
                    "{test.name}"
               }

               td { class: "px-6 py-3 text-sm",
                    "{test.closed}"
               }

               td { class: "px-6 py-3 text-sm cursor-pointer",
                    "dummy link"
               }
           }

           if *drop_down.get() && !is_empty {
               rsx! {
                tr {
                    table { class: "min-w-full bg-white font-[sans-serif]",
                        thead { class: "whitespace-nowrap",
                            th { class: "px-6 py-3 text-left text-sm font-semibold text-black",
                                "Name"
                            }
                            th { class: "px-6 py-3 text-center text-sm font-semibold text-black",
                                "Level"
                            }

                            results_rendered
                        }
                    }
                }
                }
           }
    })
}

#[inline_props]
fn ResultComponent<'a>(cx: Scope, result: &'a StudentResult) -> Element {
    cx.render(rsx! {
        tr {
            td { class: "px-6 py-3 text-sm",
                "{result.name}"
            }
            td { class: "px-6 py-3 text-center text-sm",
                "{result.level}"
            }
        }
    })
}

#[inline_props]
fn QRCodeGenerator<'a>(cx: Scope, id: &'a str) -> Element {
    None
}
