pub mod auth;
pub mod dashboard;
pub mod test;
use serde::{Deserialize, Serialize};
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
