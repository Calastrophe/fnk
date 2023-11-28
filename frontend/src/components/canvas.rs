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
    let c_width = (window.inner_width().unwrap().as_f64().unwrap() / 1.35) as i64;
    let c_height = (window.inner_height().unwrap().as_f64().unwrap() / 1.35) as i64;

    let pressed = use_state(cx, || false);

    let event_handler = move |event: Event| {
        match event {
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

        }
     };

    let enable_eraser = move |_: MouseEvent| {    
        let context = get_context();
        let _ = context.set_global_composite_operation("destination-out");
        context.set_line_width(10.0);
    };

    let enable_pen = move |_: MouseEvent| {
        let context = get_context();
        let _ = context.set_global_composite_operation("source-over");
        context.set_line_width(1.0);
    };

    let clear_canvas = move |e: MouseEvent| {
        let context = get_context();
        cx.props.onclear.call(e);
        context.clear_rect(0.0, 0.0, c_width as f64, c_height as f64);
    };

    cx.render(rsx! {
        canvas { 
            id: "drawing-box", 
            height: c_height, 
            width: c_width, 
            style: "border: solid; position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); max-width: 100%;",
            onmousedown: move |event| event_handler(Event::MouseDown(event)),
            onmousemove: move |event| event_handler(Event::MouseMove(event)),
            onmouseup: move |event| event_handler(Event::MouseUp(event)),
        }
        button { onclick: enable_eraser }
        button { onclick: enable_pen }
        button { onclick: clear_canvas }
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
