use super::{handle_response, handle_response_unit, APIError, CreateTest};
use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
pub struct Test {
    pub id: String,
    pub teacher_id: String,
    pub name: String,
    pub closed: bool,
}

#[derive(Deserialize, PartialEq)]
pub struct StudentResult {
    pub id: String,
    pub test_id: String,
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

pub async fn inverse_closed(id: &str) -> Result<(), APIError> {
    let response = reqwest::Client::new()
        .post(format!("http://localhost:8080/v1/test/{id}/manage"))
        .send()
        .await?;

    handle_response_unit(response).await
}

pub async fn get_results(id: &str) -> Result<Vec<StudentResult>, APIError> {
    let response = reqwest::Client::new()
        .get(format!("http://localhost:8080/v1/test/{id}/manage"))
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
