use gloo_net::http::Request;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hello-server")]
    HelloServer,
    #[at("/login")]
    Login,
    #[at("/drawtest")]
    DrawTest,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::HelloServer => html! { <HelloServer /> },
        Route::Login => html! { <Login /> },
        Route::DrawTest => html! { <DrawTest /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component(HelloServer)]
fn hello_server() -> Html {
    let data = use_state(|| None);

    // Request `/api/hello` once
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/hello").send().await.unwrap();
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.text().await.map_err(|err| err.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>{"Got server response: "}{data}</div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }
}

#[function_component(Login)]
fn login() -> Html {
    html! {
        <>
            <div>
                <div class="row m-2">
                    <div class="col">
                        <h3 class="text-center">{"Login"}</h3>
                        <h1 class="text-center">{"FNK"}</h1>
                    </div>
                </div>
                <form>
                    <div class="mb-3 row">
                        <div class="col-3"></div>
                        <div class="col-6">
                            <label for="username" class="form-label">{"Username"}</label>
                            <input class="form-control" id="username" type="text" aria-describedy="username-input" />
                            <div id="username-input" class="form-text">{"username you made"}</div>
                        </div>
                        <div class="col-3"></div>
                    </div>
                    <div class="mb-3 row">
                        <div class="col-3"></div>
                        <div class="col-6">
                            <label for="password" class="form-label">{"Password"}</label>
                            <input id="password" type="password" class="form-control" />
                        </div>
                        <div class="col-3"></div>
                    </div>
                    <div class="row">
                        <div class="col-3"></div>
                        <div class="col-6">
                            <button type="submit" class="btn btn-primary">{"Login"}</button>
                        </div>
                        <div class="col-3"></div>
                    </div>
                </form>
            </div>
        </>
    }
}

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
fn drawtest() -> Html {
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
            <canvas width="200" height="100"></canvas>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
