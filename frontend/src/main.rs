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
    #[at("/register")]
    Register,
    #[at("/login")]
    Login,
    
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
            
            <div style="background-color: #afeeee; padding: 20px; text-align: center; border-radius: 10px; margin: 20px;">
            <h1 style="font-family: 'Comic Neue', cursive; font-size: 3em; color: #333333; margin: 0; padding: 0;">
                { "Welcome to Fun Nd' Knowledge!" }
            </h1>
            <p style="font-family: 'Comic Neue', cursive; font-size: 1.5em; color: #555555; margin-top: 10px;">
                { "A place where learning never stops being fun." }
            </p>
        </div>
        <div class="home-content">
            
        </div>
        <BrowserRouter>
        <div style="text-align: center; margin-top: 50px;">
            <Link<Route> to={Route::Login}>
                <button style="font-size: 20px; padding: 10px 20px; background-color: #4CAF50; color: white; margin: 10px; border: none; border-radius: 5px; cursor: pointer;">
                    { "Login" }
                </button>
            </Link<Route>>
            <Link<Route> to={Route::Register}>
                <button style="font-size: 20px; padding: 10px 20px; background-color: #1E90FF; color: white; margin: 10px; border: none; border-radius: 5px; cursor: pointer;">
                    { "Register" }
                </button>
            </Link<Route>>
        </div>
       
    </BrowserRouter>
    <TeacherDashboard />
</>
},
        Route::HelloServer => html! { <HelloServer /> },
        Route::Register => html! { <Register /> },
        Route::Login => html! { <Login /> },
    }
}


#[function_component(TeacherDashboard)]
fn teacher_dashboard() -> Html {
    let student_toggle = {
        let student_dropdown_visible = use_state(|| false);
        move |_| {
            student_dropdown_visible.set(!*student_dropdown_visible);
        }
    };
    html! {
        <>
          
            
        <style>{"\n
                #teacher-dashboard-container {
                clear: both; // Ensures that this container does not collide with floating elements above.
                padding-top: 20px; // Adds space between this container and elements above.
                }
                #teacher-dashboard {
                    width: 80%;
                    text-align: center;
                    background: #ffffff;
                    padding: 15px;
                    border-radius: 8px;
                    box-shadow: 0 3px 10px rgba(0, 0, 0, 0.1);
                }
        
                #create-test-btn {
                    background-color: #4CAF50;
                    color: white;
                    border: none;
                    padding: 10px 20px;
                    border-radius: 4px;
                    cursor: pointer;
                    margin-bottom: 20px;
                    float: right;
                }
        
                .student-bar {
                    padding: 10px;
                    margin-bottom: 10px;
                    background-color: #e9eff5;
                    cursor: pointer;
                    position: relative;
                    width: 50%;
                    margin-left: 25%;
                    border-radius: 6px;
                    transition: background-color 0.2s;
                }
        
                .student-bar:hover {
                    background-color: #dce7ed;
                }
        
                .student-dropdown {
                    display: none;
                    border: 1px solid #d1d5da;
                    background-color: #ffffff;
                    width: 50%;
                    margin-left: 25%;
                    margin-bottom: 10px;
                    border-radius: 6px;
                    box-shadow: 0 3px 10px rgba(0, 0, 0, 0.1);
                }
        
                .student-dropdown a {
                    padding: 10px;
                    display: block;
                    text-decoration: none;
                    color: black;
                    border-top: 1px solid #d1d5da;
                }
        
                .student-dropdown a:first-child {
                    border-top: none;
                }
        
                .student-dropdown a:hover {
                    background-color: #dce7ed;
                }
        
                .test-name-input {
                    border: none;
                    border-bottom: 1px solid #000;
                    background: transparent;
                    margin-left: 5px;
                    outline: none;
                    transition: border 0.2s;
                }
        
                .test-name-input:focus {
                    border-bottom-color: #4CAF50;
                }
        
                /* For mobile phones: */
                @media only screen and (max-width: 600px) {
                    #teacher-dashboard {
                        width: 95%;
                    }
        
                    .student-bar,
                    .student-dropdown {
                        width: 100%;
                        margin-left: 0;
                    }
                }
        
                /* For tablets: */
                @media only screen and (min-width: 601px) and (max-width: 992px) {
                    #teacher-dashboard {
                        width: 90%;
                    }
        
                    .student-bar,
                    .student-dropdown {
                        width: 70%;
                        margin-left: 15%;
                    }
                }
        
                /* For desktops: */
                @media only screen and (min-width: 993px) {
                    #teacher-dashboard {
                        width: 80%;
                    }
        
                    .student-bar,
                    .student-dropdown {
                        width: 50%;
                        margin-left: 25%;
                    }
                }
        
                "}</style>
        
            
            <div id="teacher-dashboard">
                <button id="create-test-btn" onclick={student_toggle.clone()}>{"Create Test"}</button>
                
            </div>
        </>
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

#[function_component(Register)]
fn register() -> Html {
    unimplemented!()
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
            <div style="background-color: #afeeee; padding: 20px; text-align: center; border-radius: 10px; margin: 20px;">
                <h1 style="font-family: 'Comic Neue', cursive; font-size: 3em; color: #333333; margin: 0; padding: 0;">
                    { "Login to Fun Nd' Knowledge!" }
                </h1>
                <p style="font-family: 'Comic Neue', cursive; font-size: 1.5em; color: #555555; margin-top: 10px;">
                    { "Enter your details below." }
                </p>
                <form>
                    <div class="mb-3 row">
                        <div class="col-3"></div>
                        <div class="col-6">
                            <label for="username" class="form-label">{"Username"}</label>
                            <input class="form-control" id="username" type="text" aria-describedy="username-help" />
                            <div id="username-help" class="form-text">{"Your username"}</div>
                        </div>
                        <div class="col-3"></div>
                    </div>
                    <div class="mb-3 row">
                        <div class="col-3"></div>
                        <div class="col-6">
                            <label for="password" class="form-label">{"Password"}</label>
                            <input id="password" type="password" class="form-control" aria-describedy="password-help" />
                            <div id="password-help" class="form-text">{"Your password"}</div>
                        </div>
                        <div class="col-3"></div>
                    </div>
                    <div class="row">
                        <div class="col-3"></div>
                        <div class="col-6">
                            <button type="submit" class="btn btn-primary" style="font-size: 20px; padding: 10px 20px; background-color: #4CAF50; color: white; margin: 10px; border: none; border-radius: 5px; cursor: pointer;">{"Login"}</button>
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
