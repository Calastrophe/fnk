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

#[derive(Props)]
pub struct CanvasProps<'a> {
    ondraw: EventHandler<'a, MouseEvent>,
    onclear: EventHandler<'a, MouseEvent>,
}

pub fn Canvas<'a>(cx: Scope<'a, CanvasProps<'a>>) -> Element {
    let window = web_sys::window().unwrap();

    // TODO: Have these inside a `use_effect` and query them to update the size of the drawing.
    let c_width = (window.inner_width().unwrap().as_f64().unwrap() / 1.10) as i64;
    let c_height = (window.inner_height().unwrap().as_f64().unwrap() / 1.5) as i64;

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
        Event::MouseDown(e) => {
            pressed.set(true);
            cx.props.ondraw.call(e);
            let context = get_context();
            context.begin_path();
        }
    };

    let clear_canvas = move |e: MouseEvent| {
        let context = get_context();
        cx.props.onclear.call(e);
        context.clear_rect(0.0, 0.0, c_width as f64, c_height as f64);
    };

    cx.render(rsx! {
            div { class: "flex flex-col justify-center",
                div { class: "flex flex-row justify-center",
                    button { class: "text-white bg-gray-800 hover:bg-gray-900 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700",
                        onclick: clear_canvas,
                        "Clear"
                    }
                }

                canvas { class: "place-self-center",
                    id: "drawing-box",
                    height: c_height,
                    width: c_width,
                    style: "border: solid;",
                    onmousedown: move |event| event_handler(Event::MouseDown(event)),
                    onmousemove: move |event| event_handler(Event::MouseMove(event)),
                    onmouseup: move |event| event_handler(Event::MouseUp(event)),
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
