use crate::api::{
    auth::{login_teacher, register_teacher},
    APIError,
};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn Login(cx: Scope) -> Element {
    let resp_text = use_state(cx, || None::<String>);
    let nav = use_navigator(cx);

    let onsubmit = move |evt: FormEvent| {
        to_owned![resp_text];
        to_owned![nav];
        cx.spawn(async move {
            let resp = login_teacher(
                evt.values["email"][0].as_str(),
                evt.values["password"][0].as_str(),
            )
            .await;

            match resp {
                Err(e) => match e {
                    APIError::Validation(validation_errs) => {
                        resp_text.set(Some(validation_errs.get(0).unwrap().to_string()))
                    }
                    _ => resp_text.set(Some(e.to_string())),
                },

                Ok(_) => {
                    nav.push(crate::Route::Dashboard {});
                }
            }
        });
    };

    let (visible, err) = match resp_text.get() {
        Some(v) => (true, v.as_str()),
        None => (false, ""),
    };

    cx.render(rsx! {
        div { class: "bg-gray-50 font-[sans-serif] text-[#333]",
            div { class: "min-h-screen flex flex-col items-center justify-center py-6 px-4",
                div { class: "max-w-md w-full border py-8 px-6 rounded border-gray-300 bg-white",
                    h2 { class: "text-center text-3xl font-extrabold",
                        "Login to your account"
                    }
                    form { class: "mt-10 space-y-4",
                        onsubmit: onsubmit,
                        style: "display: flex; flex-direction: column; gap: 10px;",
                        input { class: "px-4 py-3 bg-gray-100 w-full text-sm outline-[#333] rounded",
                            r#type: "text",
                            placeholder: "Enter your email",
                            name: "email"
                        }
                        input { class: "px-4 py-3 bg-gray-100 w-full text-sm outline-[#333] rounded",
                            r#type: "password",
                            placeholder: "Enter your password",
                            name: "password"
                        }
                        div { class: "!mt-10",
                            button { class: "w-full py-2.5 px-4 text-sm rounded text-white bg-blue-600 hover:bg-blue-700 focus:outline-none",
                                "Log in"
                            }
                        }
                        if visible {
                            rsx! {
                                div { class: "p-4 mb-4 text-sm text-red-800 rounded-lg bg-red-50 dark:bg-gray-800 dark:text-red-400",
                                    span { class: "font-medium",
                                        "Invalid! "
                                    }
                                    "{err}"
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

pub fn Register(cx: Scope) -> Element {
    let resp_text = use_state(cx, || None::<String>);
    let nav = use_navigator(cx);

    let onsubmit = move |evt: FormEvent| {
        to_owned![resp_text];
        to_owned![nav];
        cx.spawn(async move {
            if evt.values["password"][0].as_str() != evt.values["c_password"][0].as_str() {
                return resp_text.set(Some("The provided passwords do not match.".to_string()));
            }

            let resp = register_teacher(
                evt.values["email"][0].as_str(),
                evt.values["username"][0].as_str(),
                evt.values["password"][0].as_str(),
            )
            .await;

            match resp {
                Err(e) => match e {
                    APIError::Validation(validation_errs) => {
                        resp_text.set(Some(validation_errs.get(0).unwrap().to_string()))
                    }
                    _ => resp_text.set(Some(e.to_string())),
                },

                Ok(_) => {
                    nav.push(crate::Route::Login {});
                }
            }
        });
    };

    let (visible, err) = match resp_text.get() {
        Some(v) => (true, v.as_str()),
        None => (false, ""),
    };

    cx.render(rsx! {
        div { class: "bg-gray-50 font-[sans-serif] text-[#333]",
            div { class: "min-h-screen flex flex-col items-center justify-center py-6 px-4",
                div { class: "max-w-md w-full border py-8 px-6 rounded border-gray-300 bg-white",
                    h2 { class: "text-center text-3xl font-extrabold",
                        "Create an account"
                    }
                    form { class: "mt-10 space-y-4",
                        onsubmit: onsubmit,
                        style: "display: flex; flex-direction: column; gap: 10px;",
                        input { class: "px-4 py-3 bg-gray-100 w-full text-sm outline-[#333] rounded",
                            r#type: "text",
                            placeholder: "Enter a email",
                            name: "email"
                        }
                        input { class: "px-4 py-3 bg-gray-100 w-full text-sm outline-[#333] rounded",
                            r#type: "username",
                            placeholder: "Enter a username",
                            name: "username"
                        }
                        input { class: "px-4 py-3 bg-gray-100 w-full text-sm outline-[#333] rounded",
                            r#type: "password",
                            placeholder: "Enter a password",
                            name: "password"
                        }
                        input { class: "px-4 py-3 bg-gray-100 w-full text-sm outline-[#333] rounded",
                            r#type: "password",
                            placeholder: "Confirm previous password",
                            name: "c_password"
                        }
                        div { class: "!mt-10",
                            button { class: "w-full py-2.5 px-4 text-sm rounded text-white bg-blue-600 hover:bg-blue-700 focus:outline-none",
                                "Register"
                            }
                        }
                        if visible {
                            rsx! {
                                div { class: "p-4 mb-4 text-sm text-red-800 rounded-lg bg-red-50 dark:bg-gray-800 dark:text-red-400",
                                    span { class: "font-medium",
                                        "Invalid! "
                                    }
                                    "{err}"
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}
