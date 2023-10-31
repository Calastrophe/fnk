use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Teacher {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct StudentResult {
    pub id: uuid::Uuid,
    pub test: uuid::Uuid,
    pub name: String,
    pub score: i32,
    pub finished: bool,
    pub flagged: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisterTeacher {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginTeacher {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterStudent {
    pub name: String,
    pub test_id: uuid::Uuid,
}
