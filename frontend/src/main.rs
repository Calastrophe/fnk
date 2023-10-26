use gloo_net::http::Request;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
//use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
//use web_sys::window;
//use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use yew_router::prelude::*;

// const MAX_WIDTH: u32 = 800;
// const MAX_HEIGHT: u32 = 400;

// #[function_component(DrawTest)]
// fn app_model() -> Html {
//     let MAX_WIDTH_str = MAX_WIDTH.to_string();
//     let MAX_HEIGHT_str = MAX_HEIGHT.to_string();

//     html! {
//         <>
//             <svg
//                 width = {MAX_WIDTH_str}
//                 height = {MAX_HEIGHT_str}
//                 version="1.1"
//                 xmlns="http://www.w3.org/2000/svg"
//             >
//                 {draw_path()}
//             </svg>
//         </>
//     }
// }

// fn draw_path(path: &String) -> Html {
//     let draw_path = if !path.is_empty() {
//         html! {
//             <path d = {path.clone()} stroke="blue" fill="none" />
//         }
//     } else {
//         html! {}
//     };

//     draw_path
// }

// pub struct AppModel2 {
//     link: ComponentLink<Self>,
//     mouse_down: bool,
//     path: String,
//     cx: f64,
//     cy: f64,
// }

// pub enum Msg {
//     StartDrawing,
//     ContinueDrawing(f64, f64),
//     StopDrawing,
//     UpdateView,
// }

// impl Component for DrawTest {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//         AppModel2 {
//             link,
//             mouse_down: false,
//             path: String::from(""),
//             cx: 0.0,
//             cy: 0.0,
//         }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
//             Msg::StartDrawing => {
//                 self.mouse_down = true;
//                 self.path = format!("M{} {}", self.cx, self.cy);
//             }
//             Msg::ContinueDrawing(x, y) => {
//                 if self.mouse_down {
//                     self.path.push_str(&format!(" L{} {}", x, y));
//                     self.cx = x;
//                     self.cy = y;
//                 }
//             }
//             Msg::StopDrawing => {
//                 self.mouse_down = false;
//             }
//             Msg::UpdateView => {
//                 // Do something when updating the view, if needed
//             }
//         }
//         true
//     }

//     fn changed(&mut self, _props: Self::Properties) -> ShouldRender {
//         false
//     }

//     fn view(&self) -> Html {
//         draw_path(&self.path)
//     }
// }

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

// #[function_component(DrawTest)]
// fn drawtest() -> Html {
//     let canvas: HtmlCanvasElement = web_sys::window()
//         .unwrap()
//         .document()
//         .unwrap()
//         .get_element_by_id("myCanvas")
//         .unwrap()
//         .unchecked_into();

//     // Get the 2D rendering context
//     let ctx: CanvasRenderingContext2d = canvas
//         .get_context("2d")
//         .unwrap()
//         .unwrap()
//         .dyn_into()
//         .unwrap();

//     // Set canvas dimensions
//     let canvas_offset_x = canvas.offset_left();
//     let canvas_offset_y = canvas.offset_top();
//     canvas.set_width(window().unwrap().inner_width().into() - canvas_offset_x);
//     canvas.set_height(window().unwrap().inner_height().into() - canvas_offset_y);

//     let is_painting = Rc::new(RefCell::new(false));
//     let line_width = Rc::new(RefCell::new(5));
//     let start_x = Rc::new(RefCell::new(0));
//     let start_y = Rc::new(RefCell::new(0));

//     let closure = Closure::wrap(Box::new(|| {
//         e.prevent_default();

//         let is_painting = is_painting.borrow().clone();
//         let line_width = line_width.borrow().clone();

//         if is_painting {
//             ctx.begin_path();
//             ctx.set_line_width(line_width as f64);
//             ctx.set_line_cap("round");
//             ctx.line_to(
//                 e.client_x() as f64 - canvas_offset_x as f64,
//                 e.client_y() as f64 - canvas_offset_y as f64,
//             );
//             ctx.stroke();
//         }
//     }));

//     canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref());
//     closure.forget();

//     html! {
//         <canvas id="myCanvas" width="200" height="100" style="border:1px solid #000000;"></canvas>
//     }
// }

fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;
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

    Ok(())
}

#[function_component(DrawTest)]
fn drawtest() -> Html {
    start().expect("Error starting canvas");

    html! {
        <>
            <div>
                <canvas id="myCanvas" width="200" height="100" style="border:1px solid #000000;"></canvas>
            </div>
        </>
    }
}


fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
}
