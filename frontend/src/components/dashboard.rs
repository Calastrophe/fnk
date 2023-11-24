use dioxus::prelude::*;

pub fn Dashboard(cx: Scope) -> Element {
    // make a request to grab the tests to see if the person is logged in

    // have a button to refetch tests AND refetch when a new one is made

    // if you click on a test, it drops down results

    // OPTIONAL: Have a side viewing of a student result on the right side if they click on it

    cx.render(rsx! {
        div { "todo" }
    })
}

fn Tests(cx: Scope) -> Element {
    render! {
        div { "todo" }
    }
}

fn StudentResults(cx: Scope) -> Element {
    render! {
        div { "todo" }
    }
}
