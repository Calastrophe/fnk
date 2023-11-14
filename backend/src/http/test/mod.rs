use axum::http::StatusCode;
use axum::middleware;
use axum::response::IntoResponse;
use axum::{routing::post, Extension, Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::http::auth::teacher_auth;
use crate::http::{Error, Result};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::http::teacher::Teacher;
pub mod student;

pub fn router() -> Router {
    Router::new()
        .route(
            "/v1/test",
            post(create_test)
                .get(get_tests)
                .route_layer(middleware::from_fn(teacher_auth)),
        )
        .route(
            "/v1/test/close",
            post(close_test).route_layer(middleware::from_fn(teacher_auth)),
        )
        .merge(student::router())
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Test {
    pub test_id: Uuid,
    pub teacher_id: Uuid,
    pub name: String,
    pub closed: bool,
}

#[derive(Deserialize, Serialize, Validate)]
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
    Extension(teacher): Extension<Teacher>,
    Json(req): Json<CreateTest>,
) -> Result<impl IntoResponse> {
    req.validate()?;

    let CreateTest { name } = req;

    let _ = sqlx::query!(
        "INSERT INTO test (teacher_id, name) VALUES ($1, $2)",
        teacher.teacher_id,
        name
    )
    .execute(&db)
    .await?;

    Ok(StatusCode::ACCEPTED)
}

async fn get_tests(
    Extension(db): Extension<PgPool>,
    Extension(teacher): Extension<Teacher>,
) -> Result<Json<Vec<Test>>> {
    let tests = sqlx::query_as!(
        Test,
        "SELECT * FROM test WHERE teacher_id = $1",
        teacher.teacher_id
    )
    .fetch_all(&db)
    .await?;

    Ok(Json(tests))
}

async fn close_test(
    Extension(db): Extension<PgPool>,
    Extension(teacher): Extension<Teacher>,
    Json(req): Json<CloseTest>,
) -> Result<StatusCode> {
    let test = sqlx::query!(
        "UPDATE test SET closed = true WHERE test_id = $1 AND teacher_id = $2",
        req.test_id,
        teacher.teacher_id,
    )
    .execute(&db)
    .await?;

    Ok(StatusCode::ACCEPTED)
}
