use crate::api::{
    dashboard::{get_results, get_tests, inverse_closed, StudentResult, Test},
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
                            "Open"
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

    let close = move |_: FormEvent| {
        to_owned![test.id];
        cx.spawn(async move {
            let _ = inverse_closed(&id).await;
        });
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
                label { class: "relative inline-flex items-center mb-5 cursor-pointer",
                    input {
                        oninput: close,
                        r#type: "checkbox",
                        checked: "{!test.closed}",
                        name: "toggle",
                        "{test.closed}"
                    }
                }
               }
               td { class: "px-6 py-3 text-sm cursor-pointer",
                    Link { id: &test.id }
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
fn Link<'a>(cx: Scope, id: &'a str) -> Element {
    cx.render(rsx! {
        a { class: "inline-flex items-center px-5 py-2.5 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800",
            href: "http://localhost:8080/test/{id}/",
            "Navigate to the test",
                svg { class: "w-4 h-4 rtl:rotate-180",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 14 10",
                path { 
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M1 5h12m0 0L9 1m4 4L9 9",
                }

            }
        }
    })
}




