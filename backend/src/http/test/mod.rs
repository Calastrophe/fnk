use axum::http::StatusCode;
use axum::{
    extract::Path, middleware, response::IntoResponse, routing::post, Extension, Json, Router,
};
use sqlx::PgPool;
use uuid::Uuid;

use self::student::StudentResult;
use crate::http::auth::teacher_auth;
use crate::http::teacher::Teacher;
use crate::http::Result;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
            "/v1/test/:test_id/manage",
            post(inverse_closed)
                .get(get_results)
                .route_layer(middleware::from_fn(teacher_auth)),
        )
        .merge(student::router())
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Test {
    pub id: Uuid,
    pub teacher_id: Uuid,
    pub name: String,
    pub closed: bool,
}

#[derive(Deserialize, Validate)]
pub struct CreateTest {
    #[validate(length(
        min = 3,
        max = 40,
        message = "The test's name must be between 3 and 40 characters long"
    ))]
    name: String,
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
        teacher.id,
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
    let tests = sqlx::query_as!(Test, "SELECT * FROM test WHERE teacher_id = $1", teacher.id)
        .fetch_all(&db)
        .await?;

    Ok(Json(tests))
}

async fn get_results(
    Extension(db): Extension<PgPool>,
    Extension(teacher): Extension<Teacher>,
    Path(test_id): Path<Uuid>,
) -> Result<Json<Vec<StudentResult>>> {
    let results = sqlx::query_as!(
        StudentResult,
        "SELECT result.* FROM result
        JOIN test ON result.test_id = test.id
        WHERE result.test_id = $1 AND test.teacher_id = $2",
        test_id,
        teacher.id,
    )
    .fetch_all(&db)
    .await?;

    Ok(Json(results))
}

async fn inverse_closed(
    Extension(db): Extension<PgPool>,
    Extension(teacher): Extension<Teacher>,
    Path(test_id): Path<Uuid>,
) -> Result<StatusCode> {
    let _ = sqlx::query!(
        "UPDATE test SET closed = NOT closed WHERE id = $1 AND teacher_id = $2",
        test_id,
        teacher.id,
    )
    .execute(&db)
    .await?;

    Ok(StatusCode::ACCEPTED)
}
