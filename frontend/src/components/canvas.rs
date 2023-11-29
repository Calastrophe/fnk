use super::test::{Action, TestState};
use dioxus::html::MouseEvent;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

#[derive(Debug)]
enum Event {
    MouseMove(MouseEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
}

pub fn Canvas(cx: Scope) -> Element {
    let window = web_sys::window().unwrap();

    // TODO: Have these inside a `use_effect` and query them to update the size of the drawing.
    let c_width = (window.inner_width().unwrap().as_f64().unwrap() / 1.10) as i64;
    let c_height = (window.inner_height().unwrap().as_f64().unwrap() / 1.60) as i64;
    let test_state = use_shared_state::<TestState>(cx).unwrap();

    let pressed = use_state(cx, || false);

    let event_handler = move |event: Event| match event {
        Event::MouseMove(e) => {
            if *pressed.get() {
                let cords = e.element_coordinates().to_f64();
                let context = get_context();
                context.line_to(cords.x, cords.y);
                context.stroke();
                context.begin_path();
                context.move_to(cords.x, cords.y);
            }
        }
        Event::MouseUp(e) => {
            pressed.set(false);
            let context = get_context();
            let cords = e.element_coordinates().to_f64();
            context.line_to(cords.x, cords.y);
            context.stroke();
        }
        Event::MouseDown(_) => {
            pressed.set(true);
            test_state.write().has_drawn = true;
            let context = get_context();
            context.begin_path();
        }
    };

    let clear_canvas = move |_| {
        let context = get_context();
        test_state.write().has_drawn = false;
        context.clear_rect(0.0, 0.0, c_width as f64, c_height as f64);
    };

    cx.render(rsx! {
            div { class: "flex flex-col justify-center",
                // The canvas itself
                canvas { class: "place-self-center rounded-lg",
                    id: "drawing-box",
                    height: c_height,
                    width: c_width,
                    style: "border: solid;",
                    onmousedown: move |event| event_handler(Event::MouseDown(event)),
                    onmousemove: move |event| event_handler(Event::MouseMove(event)),
                    onmouseup: move |event| event_handler(Event::MouseUp(event)),
                }

                // Button for clearing canvas
                div { class: "flex flex-row justify-center",
                    button { class: "text-white bg-gray-800 rounded hover:bg-gray-900 font-medium text-sm px-5 py-2.5 me-2 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700",
                        width: "{c_width}px",
                        onclick: clear_canvas,
                        "Clear"
                    }
                }

                // Buttons for submitting
                Buttons {
                    onclear: clear_canvas
                }
            }
    })
}

#[derive(Props)]
struct ButtonsProps<'a> {
    onclear: EventHandler<'a, MouseEvent>,
}

fn Buttons<'a>(cx: Scope<'a, ButtonsProps<'a>>) -> Element {
    let test_state = use_shared_state::<TestState>(cx).unwrap();
    let submitted = use_state(cx, || false);

    cx.render(rsx! {
       if *submitted.get() {
            rsx! {
                    div { class: "flex flex-col text-center py-8",
                    span { class: "text-xl",
                    "Would you like a harder question?"
                    }
                        div {
                            button { class: "m-6 px-6 py-2 w-1/4 rounded text-white text-sm tracking-wider font-medium outline-none border-2 border-red-600 bg-red-600 hover:bg-transparent hover:text-black transition-all duration-300",
                                onclick: move |evt| {
                                    cx.props.onclear.call(evt);
                                    submitted.set(false);
                                    test_state.write().perform_action(Action::Next)
                                },
                                "No"
                            }
                            button { class: "m-6 px-6 py-2 w-1/4 rounded text-white text-sm tracking-wider font-medium outline-none border-2 border-green-600 bg-green-600 hover:bg-transparent hover:text-black transition-all duration-300",
                                onclick: move |evt| {
                                    cx.props.onclear.call(evt);
                                    submitted.set(false);
                                    test_state.write().perform_action(Action::LevelUp)
                                },
                                "Yes"
                            }
                        }
                    }
            }
        } else {
            rsx! {
                if test_state.read().has_drawn {
                    rsx! {
                        div { class: "flex justify-center py-8",
                            button { class: "px-2 py-2.5 min-w-[140px] w-2/5 bg-gradient-to-r from-green-400 rounded text-white text-sm tracking-wider font-medium border-none outline-none bg-green-600 active:from-green-500",
                                onclick: move |_| submitted.set(true),
                                "Submit"
                            }
                        }
                    }
                }
            }
        }
    })
}

fn get_context() -> CanvasRenderingContext2d {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let canvas = document
        .get_element_by_id("drawing-box")
        .expect("expecting a canvas in the document")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}
