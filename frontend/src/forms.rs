use serde::Serialize;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Serialize)]
pub struct RegisterTeacher {
    email: String,
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginTeacher {
    email: String,
    password: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let error = use_state(|| None);

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let login_data = serde_json::json!({
                "username": (*username).clone(),
                "password": (*password).clone()
            });

            let error = error.clone();
            spawn_local(async move {
                let request_result = Request::post("/v1/teacher/login")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&login_data).unwrap());
            
            match request_result {
                Ok(request) => {
                    match request.send().await {
                        Ok(response) => {
                            if response.ok() {
                                // Handle successful login, store token, update state
                            } else {
                                // Handle login error
                                error.set(Some("Invalid credentials".to_string()));
                            }
                        }
                        Err(_) => {
                            // Handle network error
                            error.set(Some("Network error".to_string()));
                        }
                    }
                }
                Err(_) => {
                    // Handle request creation error
                    error.set(Some("Request creation failed".to_string()));
                }
            }
        });
    })
};

html! {
    <>
        <div>
            <div class="row m-2">
                <div class="col">
                    <h3 class="text-center">{"Login"}</h3>
                    <h1 class="text-center">{"FNK"}</h1>
                </div>
            </div>
            <form onsubmit={onsubmit}>
                <div class="mb-3 row">
                    <div class="col-3"></div>
                    <div class="col-6">
                        <label for="username" class="form-label">{"Username"}</label>
                        <input 
                            class="form-control" 
                            type="text" 
                            placeholder="Username" 
                            onchange={Callback::from(move |e: Event| username.set(e.target_unchecked_into::<HtmlInputElement>().value()))} 
                        />
                    </div>
                    <div class="col-3"></div>
                </div>
                <div class="mb-3 row">
                    <div class="col-3"></div>
                    <div class="col-6">
                        <label for="password" class="form-label">{"Password"}</label>
                        <input 
                            class="form-control" 
                            type="password" 
                            placeholder="Password" 
                            onchange={Callback::from(move |e: Event| password.set(e.target_unchecked_into::<HtmlInputElement>().value()))} 
                        />
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
                if let Some(error_message) = (*error).clone() {
                    <div class="alert alert-danger" role="alert">
                        { error_message }
                    </div>
                }
            </form>
        </div>
    </>
}
}



#[function_component(Register)]
pub fn register() -> Html {
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let email = use_state(|| "".to_string());
    let error = use_state(|| None);

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let email = email.clone();
        let error = error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let registration_data = serde_json::json!({
                "username": (*username).clone(),
                "password": (*password).clone(),
                "email": (*email).clone(),
            });

            let error = error.clone();
            spawn_local(async move {
                let request_result = Request::post("/v1/teacher/register")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&registration_data).unwrap());

                match request_result {
                    Ok(request) => {
                        match request.send().await {
                            Ok(response) => {
                                if response.ok() {
                                    
                                } else {
                                    
                                    error.set(Some("Registration failed".to_string()));
                                }
                            }
                            Err(_) => {
                                
                                error.set(Some("Network error".to_string()));
                            }
                        }
                    }
                    Err(_) => {
                        
                        error.set(Some("Request creation failed".to_string()));
                    }
                }
            });
        })
    };
    html! {
        <>
        <style>
        
                {" 
                    #reg-container {
                        background-color: #afeeee;
                        padding: 20px;
                        text-align: center;
                        border-radius: 10px;
                        margin: 20px;
                    }
                    .reg-header {
                        font-family: 'Comic Neue', cursive;
                        color: #333333;
                        margin: 0;
                        padding: 0;
                    }
                    .reg-header h1 {
                        font-size: 3em;
                    }
                    .reg-header p {
                        font-size: 1.5em;
                        color: #555555;
                        margin-top: 10px;
                    }
                    .reg-control {
                        margin-bottom: 10px;
                    }
                    .reg-button {
                        font-size: 20px;
                        padding: 10px 20px;
                        background-color: #4CAF50;
                        color: white;
                        margin: 10px;
                        border: none;
                        border-radius: 5px;
                        cursor: pointer;
                    }
                "}
            </style>
            <div id="reg-container">
                <div class="reg-header">
                    <h1>{ "Register as a Teacher" }</h1>
                    <p>{ "Enter your registration details below." }</p>
                </div>
                <form onsubmit={onsubmit}>
                    <div class="mb-3">
                        <label for="username" class="form-label">{"Username"}</label>
                        <input
                            type="text"
                            class="form-control"
                            placeholder="Username"
                            onchange={Callback::from(move |e: Event| username.set(e.target_unchecked_into::<HtmlInputElement>().value()))}
                        />
                    </div>
                    <div class="mb-3">
                        <label for="email" class="form-label">{"Email"}</label>
                        <input
                            type="email"
                            class="form-control"
                            placeholder="Email"
                            onchange={Callback::from(move |e: Event| email.set(e.target_unchecked_into::<HtmlInputElement>().value()))}
                        />
                    </div>
                    <div class="mb-3">
                        <label for="password" class="form-label">{"Password"}</label>
                        <input
                            type="password"
                            class="form-control"
                            placeholder="Password"
                            onchange={Callback::from(move |e: Event| password.set(e.target_unchecked_into::<HtmlInputElement>().value()))}
                        />
                    </div>
                    <button type="submit" class="reg-button">{"Register"}</button>
                    if let Some(error_message) = (*error).clone() {
                        <div class="alert alert-danger" role="alert">
                            { error_message }
                        </div>
                    }
                </form>
            </div>
        </>
    }
}
