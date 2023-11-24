mod api;
mod components;
use components::{
    dashboard::Dashboard,
    forms::{Login, Register},
    test::Test,
    NotFound,
};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Dashboard {},
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/test/:id/")]
    Test { id: String },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
