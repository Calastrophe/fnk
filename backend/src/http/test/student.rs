use axum::extract::Path;
use axum::http::{header, HeaderMap, StatusCode};
use axum::middleware;
use axum::response::IntoResponse;
use axum::{routing::post, Extension, Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::http::auth::student_auth;
use crate::http::test::Test;
use crate::http::{Error, Result};
use crate::util::Config;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn router() -> Router {
    Router::new()
        .route("/v1/test/:test_id/register", post(register_student))
        .route(
            "/v1/test/:test_id/",
            post(set_score).route_layer(middleware::from_fn(student_auth)),
        )
}

#[derive(Deserialize, Validate)]
pub struct RegisterStudent {
    #[validate(length(min = 3, max = 60))]
    name: String,
}

#[derive(Deserialize, Validate)]
pub struct SetScore {
    #[validate(range(min = 0, max = 30))]
    score: i32,
}

#[derive(Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct StudentResult {
    pub id: Uuid,
    pub test_id: Uuid,
    pub name: String,
    pub score: i32,
    pub flagged: bool,
}

async fn register_student(
    Extension(db): Extension<PgPool>,
    Extension(cfg): Extension<Config>,
    Path(test_id): Path<Uuid>,
    Json(req): Json<RegisterStudent>,
) -> Result<impl IntoResponse> {
    req.validate()?;

    let RegisterStudent { name } = req;

    let test = sqlx::query_as!(Test, "SELECT * FROM test WHERE test_id = $1", test_id)
        .fetch_optional(&db)
        .await?;

    // Does the test exist?
    if let Some(test) = test {
        // Is the test closed?
        if test.closed {
            return Err(Error::Conflict(
                "The test is closed to new registration".to_string(),
            ));
        }

        // Insert the student result into the table, return on any conflicts.
        let res = sqlx::query_as!(
            StudentResult,
            "INSERT INTO result (test_id, name) VALUES ($1, $2) RETURNING *",
            test_id,
            name
        )
        .fetch_one(&db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("result_name_key") => {
                Error::Conflict("This name is already taken.".to_string())
            }
            _ => e.into(),
        })?;

        // Create a cookie for this student which correlates to the student result.
        let cookie = crate::http::auth::create_cookie("STUDENT_TOKEN", res.id, cfg).await;

        let mut headers = HeaderMap::new();
        headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

        return Ok((StatusCode::ACCEPTED, headers));
    }

    Err(Error::UnprocessableEntity(
        "The given test ID does not exist".to_string(),
    ))
}

async fn set_score(
    Extension(db): Extension<PgPool>,
    Extension(student): Extension<StudentResult>,
    Path(test_id): Path<Uuid>,
    Json(req): Json<SetScore>,
) -> Result<impl IntoResponse> {
    Ok(StatusCode::NOT_IMPLEMENTED)
}
