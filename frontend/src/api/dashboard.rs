use super::{APIError, CloseTest};
use crate::api::CreateTest;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Test {
    pub test_id: Uuid,
    pub teacher_id: Uuid,
    pub name: String,
    pub closed: bool,
}

pub async fn create_test(name: &str) -> Result<(), APIError> {
    let ret = reqwest::Client::new()
        .post("http://localhost:8080/v1/test")
        .json(&CreateTest {
            name: name.to_string(),
        })
        .send()
        .await?;

    match ret.status().is_success() {
        false => {
            let resp = ret.json::<ErrorResponse>().await?;
            let err = match resp.errors {
                Some(validation_errs) => APIError::Validation(validation_errs),
                None => APIError::ServerResponse(resp.message),
            };
            Err(err)
        }
        _ => Ok(()),
    }
}

pub async fn close_test(test_id: String) -> Result<(), APIError> {
    let ret = reqwest::Client::new()
        .post("http://localhost:8080/v1/test/close")
        .json(&CloseTest { test_id })
        .send()
        .await?;

    match ret.status().is_success() {
        false => {
            let resp = ret.json::<ErrorResponse>().await?;
            let err = match resp.errors {
                Some(validation_errs) => APIError::Validation(validation_errs),
                None => APIError::ServerResponse(resp.message),
            };
            Err(err)
        }
        _ => Ok(()),
    }
}

pub async fn get_tests() -> Result<Vec<Test>, APIError> {
    let ret = reqwest::Client::new()
        .get("http://localhost:8080/v1/test")
        .send()
        .await?;

    match ret.status().is_success() {
        false => {
            let resp = ret.json::<ErrorResponse>().await?;
            let err = match resp.errors {
                Some(validation_errs) => APIError::Validation(validation_errs),
                None => APIError::ServerResponse(resp.message),
            };
            Err(err)
        }
        _ => Ok(ret.json::<Vec<Test>>().await?),
    }
}

