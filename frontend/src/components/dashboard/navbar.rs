use crate::api::{dashboard::create_test, APIError};
use dioxus::prelude::*;

#[derive(Props)]
pub struct NavBarProps<'a> {
    onrefresh: EventHandler<'a, MouseEvent>,
}

pub fn NavBar<'a>(cx: Scope<'a, NavBarProps<'a>>) -> Element {
    cx.render(rsx! {
        nav { class: "bg-white border-gray-200 dark:bg-gray-900 dark:border-gray-700",
            div { class: "max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4",
                CreateButton {}

                button { class: "flex items-center px-4 py-2 font-medium tracking-wide text-white capitalize transition-colors duration-300 transform bg-indigo-600 rounded-lg hover:bg-indigo-500 focus:outline-none focus:ring focus:ring-indigo-300 focus:ring-opacity-80",
                    onclick: move |evt| cx.props.onrefresh.call(evt),
                    svg { class: "w-5 h-5 mx-1",
                       xmlns: "http://www.w3.org/2000/svg",
                       view_box: "0 0 20 20",
                       fill: "currentColor",
                       path {
                            fill_rule: "evenodd",
                            d: "M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z",
                            clip_rule: "evenodd",
                       }
                    }

                    span { class: "mx-1",
                        "Refetch tests"
                    }
                }
            }
        }
    })
}

fn CreateButton(cx: Scope) -> Element {
    let visible = use_state(cx, || false);
    let resp_text = use_state(cx, || None::<String>);

    let on_submit = move |evt: FormEvent| {
        to_owned![resp_text];
        to_owned![visible];

        cx.spawn(async move {
            let resp = create_test(evt.values["name"][0].as_str()).await;

            match resp {
                Err(e) => match e {
                    APIError::Validation(validation_errs) => {
                        resp_text.set(Some(validation_errs.join(",")))
                    }
                    _ => resp_text.set(Some(e.to_string())),
                },
                Ok(_) => visible.set(false),
            }
        });
    };

    let (is_err, msg) = match resp_text.get() {
        Some(v) => (true, v.as_str()),
        None => (false, ""),
    };

    cx.render(if *visible.get() {
        rsx! {
            form {
                onsubmit: on_submit,
                input { name: "name" },
                button {
                    onclick: move |_| visible.set(false),
                    "Cancel"
                }
                button {
                    "Submit"
                }
                if is_err {
                    rsx! {
                        span {
                            "{msg}"
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            button { class: "flex items-center px-4 py-2 font-medium tracking-wide text-white capitalize transition-colors duration-300 transform bg-indigo-600 rounded-lg hover:bg-indigo-500 focus:outline-none focus:ring focus:ring-indigo-300 focus:ring-opacity-80",
                onclick: move |_| {
                    resp_text.set(None);
                    visible.set(true)
                },
                "Create a new test"
            }
        }
    })
}
