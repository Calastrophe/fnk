use crate::api::{
    auth::{login_teacher, register_teacher},
    APIError,
};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn Login(cx: Scope) -> Element {
    let resp_text = use_state(cx, || String::new());
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
                        resp_text.set(validation_errs.join(", "))
                    }
                    _ => resp_text.set(e.to_string()),
                },

                Ok(_) => {
                    nav.push(crate::Route::Dashboard {});
                }
            }
        });
    };

    render! {
            h1 { "Login" }
            form {
                onsubmit: onsubmit,
                div { "Email: " }
                input { r#type: "text", name: "email" }
                br {}
                div { "Password: " }
                input { r#type: "password", name: "password" }
                br {}
                br {}
                button { "Submit" }
                br {}
                div { "{resp_text}" }
            }
    }
}

pub fn Register(cx: Scope) -> Element {
    let resp_text = use_state(cx, || String::new());
    let nav = use_navigator(cx);

    let onsubmit = move |evt: FormEvent| {
        to_owned![resp_text];
        to_owned![nav];
        cx.spawn(async move {
            let resp = register_teacher(
                evt.values["email"][0].as_str(),
                evt.values["username"][0].as_str(),
                evt.values["password"][0].as_str(),
            )
            .await;

            match resp {
                Err(e) => match e {
                    APIError::Validation(validation_errs) => {
                        resp_text.set(validation_errs.join(", "))
                    }
                    _ => resp_text.set(e.to_string()),
                },

                Ok(_) => {
                    nav.push(crate::Route::Dashboard {});
                }
            }
        });
    };

    render! {
            h1 { "Register" }
            form {
                onsubmit: onsubmit,
                div { "Email: " }
                input { r#type: "text", name: "email" }
                br {}
                div { "Username: " }
                input { r#type: "text", name: "username" }
                br {}
                div { "Password: " }
                input { r#type: "password", name: "password" }
                br {}
                br {}
                button { "Submit" }
                br {}
                div { "{resp_text}" }
            }
    }
}
