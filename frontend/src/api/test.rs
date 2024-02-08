use super::{handle_response, handle_response_unit, APIError, RegisterStudent, SetLevel, API_URL};

#[derive(serde::Deserialize, PartialEq)]
pub struct Question {
    pub id: String,
    pub level: i32,
    pub question: String,
    pub image_path: Option<String>,
}

pub async fn get_questions(level: i32) -> Result<Vec<Question>, APIError> {
    let response = reqwest::Client::new()
        .get(format!("{API_URL}/question/{level}"))
        .send()
        .await?;

    handle_response(response).await
}

pub async fn register_student(id: &str, name: &str) -> Result<(), APIError> {
    let response = reqwest::Client::new()
        .post(format!("{API_URL}/test/{id}/register"))
        .json(&RegisterStudent {
            name: name.to_string(),
        })
        .send()
        .await?;

    handle_response_unit(response).await
}

pub async fn set_level(id: &str, level: i32) -> Result<(), APIError> {
    let response = reqwest::Client::new()
        .post(format!("{API_URL}/test/{id}"))
        .json(&SetLevel { level })
        .send()
        .await?;

    handle_response_unit(response).await
}
