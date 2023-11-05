use axum::http::{header, HeaderMap, StatusCode};
use axum::middleware;
use axum::response::IntoResponse;
use axum::{routing::post, Extension, Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::http::auth::teacher_auth;
use crate::http::{Error, Result};
use serde::{Deserialize, Serialize};
use validator::Validate;
pub mod student;

pub fn router() -> Router {
    Router::new()
        .route(
            "/v1/test",
            post(create_test)
                .post(close_test)
                .route_layer(middleware::from_fn(teacher_auth)),
        )
        .merge(student::router())
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Test {
    test_id: Uuid,
    teacher_id: Uuid,
    name: String,
    closed: bool,
}

#[derive(Deserialize, Validate)]
pub struct CreateTest {
    #[validate(length(min = 3, max = 60))]
    name: String,
}

#[derive(Deserialize)]
pub struct CloseTest {
    test_id: uuid::Uuid,
}

async fn create_test(
    Extension(db): Extension<PgPool>,
    Json(req): Json<CreateTest>,
) -> Result<impl IntoResponse> {
    req.validate()?;

    let CreateTest { name } = req;

    Ok(StatusCode::NO_CONTENT)
}

async fn close_test(db: Extension<PgPool>, Json(req): Json<CloseTest>) -> Result<StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}
