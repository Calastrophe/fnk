use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{routing::post, Extension, Json, Router};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use std::time::Duration;

use super::auth::TokenClaims;
use crate::http::{Error, Result};
use crate::util::Config;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn router() -> Router {
    Router::new().route("/v1/student", post(register_student))
}

#[derive(Deserialize, Validate)]
pub struct RegisterStudent {
    #[validate(length(min = 3, max = 60))]
    name: String,
    test_id: uuid::Uuid,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct StudentResult {
    pub id: uuid::Uuid,
    pub test_id: uuid::Uuid,
    pub name: String,
    pub score: i32,
    pub finished: bool,
    pub flagged: bool,
}

async fn register_student(
    db: Extension<PgPool>,
    cfg: Extension<Config>,
    Json(req): Json<RegisterStudent>,
) -> Result<impl IntoResponse> {
    req.validate()?;

    let RegisterStudent { name, test_id } = req;

    // Query to ensure the test exists and isn't closed

    // If it exists and isn't closed, register the student, add to the table, and give a cookie.

    Ok(StatusCode::NO_CONTENT)
}
