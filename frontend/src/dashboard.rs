use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <>     
            <div style="display: flex; flex-direction: row; justify-content: space-between; align-items: center; background-color: #afeeee; padding: 20px; text-align: center; border-radius: 10px; margin: 20px;">
                <h1 style="font-family: 'Comic Neue', cursive; font-size: 3em; color: #333333; margin: 0; padding: 0;">
                    { "Welcome to Fun Nd' Knowledge!" }
                </h1>
                <div style="text-align: right;">
                    <BrowserRouter>
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
                    </BrowserRouter>
                </div>
            </div>
            <div class="home-content">
                // ... other content
            </div>
            <TeacherDashboard />
    
    
</>
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
                clear: both; 
                padding-top: 20px; 
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