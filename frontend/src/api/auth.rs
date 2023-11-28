use super::{
    handle_response_unit, APIError, ErrorResponse, LoginTeacher, RegisterStudent, RegisterTeacher,
};

pub async fn register_teacher(email: &str, username: &str, password: &str) -> Result<(), APIError> {
    let response = reqwest::Client::new()
        .post("http://localhost:8080/v1/teacher/register")
        .json(&RegisterTeacher {
            email: email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        })
        .send()
        .await?;

    handle_response_unit(response).await
}

pub async fn login_teacher(email: &str, password: &str) -> Result<(), APIError> {
    let response = reqwest::Client::new()
        .post("http://localhost:8080/v1/teacher/login")
        .json(&LoginTeacher {
            email: email.to_string(),
            password: password.to_string(),
        })
        .send()
        .await?;

    handle_response_unit(response).await
}
