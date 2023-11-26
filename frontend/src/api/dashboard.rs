use super::{
    handle_response, handle_response_unit, APIError, CloseTest, CreateTest, ErrorResponse,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Test {
    pub id: Uuid,
    pub teacher_id: Uuid,
    pub name: String,
    pub closed: bool,
}

#[derive(Deserialize)]
pub struct StudentResult {
    pub id: Uuid,
    pub test_id: Uuid,
    pub name: String,
    pub level: i32,
}

pub async fn create_test(name: &str) -> Result<(), APIError> {
    let response = reqwest::Client::new()
        .post("http://localhost:8080/v1/test")
        .json(&CreateTest {
            name: name.to_string(),
        })
        .send()
        .await?;

    handle_response_unit(response).await
}

pub async fn close_test(test_id: Uuid) -> Result<(), APIError> {
    let response = reqwest::Client::new()
        .post(format!("http://localhost:8080/v1/{test_id}/manage"))
        .send()
        .await?;

    handle_response_unit(response).await
}

pub async fn get_results(test_id: Uuid) -> Result<Vec<StudentResult>, APIError> {
    let response = reqwest::Client::new()
        .get(format!("http://localhost:8080/v1/{test_id}/manage"))
        .send()
        .await?;

    handle_response(response).await
}

pub async fn get_tests() -> Result<Vec<Test>, APIError> {
    let response = reqwest::Client::new()
        .get("http://localhost:8080/v1/test")
        .send()
        .await?;

    handle_response(response).await
}
