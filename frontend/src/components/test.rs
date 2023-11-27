use dioxus::prelude::*;

#[inline_props]
pub fn Test(cx: Scope, id: String) -> Element {
    // TODO: Potentially make this the preceding component
    // Conditional render a registration form until they are registered.
    let is_registered = use_state(cx, || false);

    // This will be sent to the server when the test is submitted
    let current_level = use_state(cx, || 1);

    // Reset current_attempt to 1 when they progress up another level
    let current_attempt = use_state(cx, || 1);

    // This has to be used by the drawing component with use_shared_state::<bool>()
    // If there needs to be more state, make a struct.
    let has_drawn = use_shared_state_provider(cx, || false);

    if *is_registered.get() {
        render! {
            div { "todo" }
        }
    } else {
        render! {
            div { "todo" }
        }
    }
}
