use super::{handle_response_unit, APIError, LoginTeacher, RegisterTeacher, API_URL};

pub async fn register_teacher(email: &str, username: &str, password: &str) -> Result<(), APIError> {
    let response = reqwest::Client::new()
        .post(format!("{API_URL}/teacher/register"))
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
        .post(format!("{API_URL}/teacher/login"))
        .json(&LoginTeacher {
            email: email.to_string(),
            password: password.to_string(),
        })
        .send()
        .await?;

    handle_response_unit(response).await
}

pub async fn logout_teacher() -> Result<(), APIError> {
    let response = reqwest::Client::new()
        .post(format!("{API_URL}/teacher/logout"))
        .send()
        .await?;

    handle_response_unit(response).await
}
