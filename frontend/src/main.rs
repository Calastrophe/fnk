use dashboard::Dashboard;
use drawing::DrawTest;
use forms::{Login, Register};
use test::TestInterface;
use yew::prelude::*;
use yew_router::prelude::*;
mod dashboard;
mod drawing;
mod forms;
mod test;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/dashboard")]
    Dashboard,
    #[at("/login")]
    Login,
    #[at("/drawtest")]
    DrawTest,
    #[at("/register")]
    Register,
    #[at("/test/:id")]
    Test { id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Dashboard => html! { <Dashboard /> },
        Route::Login => html! { <Login /> },
        Route::DrawTest => html! { <DrawTest /> },
        Route::Register => html! { <Register /> },
        Route::Test { id } => html! { <TestInterface id={id}/>},
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

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

// #[function_component(HelloServer)]
// fn hello_server() -> Html {
//     let data = use_state(|| None);
//
//     {
//         let data = data.clone();
//         use_effect(move || {
//             if data.is_none() {
//                 spawn_local(async move {
//                     let resp = Request::get("/v1/health-check").send().await.unwrap();
//                     let result = {
//                         if !resp.ok() {
//                             Err(format!(
//                                 "Error fetching data {} ({})",
//                                 resp.status(),
//                                 resp.status_text()
//                             ))
//                         } else {
//                             resp.text().await.map_err(|err| err.to_string())
//                         }
//                     };
//                     data.set(Some(result));
//                 });
//             }
//
//             || {}
//         });
//     }
//
//     match data.as_ref() {
//         None => {
//             html! {
//                 <div>{"No server response"}</div>
//             }
//         }
//         Some(Ok(data)) => {
//             html! {
//                 <div>{"Got server response: "}{data}</div>
//             }
//         }
//         Some(Err(err)) => {
//             html! {
//                 <div>{"Error requesting data from server: "}{err}</div>
//             }
//         }
//     }
// }
