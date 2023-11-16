use serde::Serialize;
use yew::prelude::*;

#[derive(Serialize)]
pub struct RegisterTeacher {
    email: String,
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginTeacher {
    email: String,
    password: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    html! {}
}

#[function_component(Register)]
pub fn register() -> Html {
    html! {}
}
