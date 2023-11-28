use crate::api::{dashboard::create_test, APIError};
use dioxus::prelude::*;

pub fn NavBar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            CreateButton { }

            button {
                "Refetch tests"
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
                Ok(_) => resp_text.set(Some("Successfully created".to_string())),
            }

            visible.set(false);
            std::thread::sleep(std::time::Duration::from_secs(2));
            resp_text.set(None);
        });
    };

    cx.render(if *visible.get() {
        rsx! {
            form {
                onsubmit: on_submit,
                input { name: "name", },
                button {
                    onclick: move |_| visible.set(false),
                    "Cancel"
                }
                button {
                    "Submit"
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
