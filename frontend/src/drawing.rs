use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use yew::prelude::*;

fn clear_canvas(canvas: &web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    Ok(())
}

fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.set_attribute("style", "border: solid; position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); max-width: 100%;")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);

    // Create a flag to keep track of the current mode: 0 for pen, 1 for eraser
    let drawing_mode = Rc::new(Cell::new(0));

    // Create a button for switching to the eraser
    let eraser_button = document.create_element("button")?;
    eraser_button.set_text_content(Some("Eraser"));
    let drawing_mode_clone = drawing_mode.clone();
    let context_clone = context.clone();
    let eraser_closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
        drawing_mode_clone.set(1); // Set mode to eraser
        context_clone.set_global_composite_operation("destination-out");
        context_clone.set_line_width(10.0); // Set the width of the eraser
    });
    eraser_button
        .add_event_listener_with_callback("click", eraser_closure.as_ref().unchecked_ref())?;
    eraser_closure.forget();
    document.body().unwrap().append_child(&eraser_button)?;

    // Create a button for switching back to the pen
    let pen_button = document.create_element("button")?;
    pen_button.set_text_content(Some("Pen"));
    let drawing_mode_clone = drawing_mode.clone();
    let context_clone = context.clone();
    let pen_closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
        drawing_mode_clone.set(0); // Set mode to pen
        context_clone.set_global_composite_operation("source-over");
        context_clone.set_line_width(1.0);
    });
    pen_button.add_event_listener_with_callback("click", pen_closure.as_ref().unchecked_ref())?;
    pen_closure.forget();
    document.body().unwrap().append_child(&pen_button)?;

    let pressed = Rc::new(Cell::new(false));
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        });
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        });
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    let clear_button = document.create_element("button")?;
    clear_button.set_text_content(Some("Clear Canvas"));

    let canvas_clone = canvas.clone();
    let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
        clear_canvas(&canvas_clone).expect("Failed to clear the canvas");
    });
    clear_button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();

    document.body().unwrap().append_child(&clear_button)?;

    Ok(())
}

#[function_component(DrawTest)]
pub fn drawtest() -> Html {
    start().expect("Error starting canvas");

    html! {
        <>
            <div>
                <div class="row m-2">
                    <div class="col">
                        <h3 class="text-center">{"Draw Test"}</h3>
                        <h1 class="text-center">{"FNK"}</h1>
                    </div>
                </div>
            </div>
        </>
    }
}