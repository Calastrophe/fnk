use super::{handle_response, APIError};

#[derive(serde::Deserialize)]
pub struct Question {
    level: i32,
    question: String,
    image_path: Option<String>,
}

pub async fn get_questions(level: i32) -> Result<Vec<Question>, APIError> {
    let response = reqwest::Client::new()
        .get(format!("http://localhost:8080/v1/question/{level}"))
        .send()
        .await?;

    handle_response(response).await
}
