pub mod auth;
pub mod dashboard;
pub mod test;
use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

// API internal types for creating / parsing JSON requests & responses

#[derive(Debug, Error)]
pub enum APIError {
    #[error("An unexpected error occurred when trying to communicate with the server")]
    ClientFailure(#[from] reqwest::Error),
    #[error("{0}")]
    ServerResponse(String),
    #[error("{0:?}")]
    Validation(Vec<String>),
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    message: String,
    errors: Option<Vec<String>>,
}

#[derive(Serialize)]
struct RegisterTeacher {
    email: String,
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginTeacher {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterStudent {
    name: String,
}

#[derive(Serialize)]
struct CreateTest {
    name: String,
}

#[derive(Serialize)]
struct CloseTest {
    test_id: uuid::Uuid,
}

#[derive(Serialize)]
struct SetScore {
    score: i32,
}

// Utility functions to handle responses.
// Separate functions because Rust doesn't have specialization.
// We could have done dynamic type checking, but rather let the compiler stretch its legs.

async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T, APIError> {
    match response.status().is_success() {
        false => {
            let resp = response.json::<ErrorResponse>().await?;
            let err = match resp.errors {
                Some(validation_errs) => APIError::Validation(validation_errs),
                None => APIError::ServerResponse(resp.message),
            };
            Err(err)
        }
        _ => Ok(response.json::<T>().await?),
    }
}

async fn handle_response_unit(response: Response) -> Result<(), APIError> {
    match response.status().is_success() {
        false => {
            let resp = response.json::<ErrorResponse>().await?;
            let err = match resp.errors {
                Some(validation_errs) => APIError::Validation(validation_errs),
                None => APIError::ServerResponse(resp.message),
            };
            Err(err)
        }
        _ => Ok(()),
    }
}
