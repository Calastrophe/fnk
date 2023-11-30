use axum::extract::Path;
use axum::http::{header, HeaderMap, StatusCode};
use axum::middleware;
use axum::response::IntoResponse;
use axum::{routing::post, Extension, Json, Router};
use axum_extra::extract::cookie::{Cookie, SameSite};
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
            "/v1/test/:test_id",
            post(set_score).route_layer(middleware::from_fn(student_auth)),
        )
}

#[derive(Deserialize, Validate)]
pub struct RegisterStudent {
    #[validate(length(
        min = 3,
        max = 40,
        message = "Your name must be between 3 and 40 characters long"
    ))]
    name: String,
}

#[derive(Deserialize, Validate)]
pub struct SetLevel {
    #[validate(range(min = 1, max = 8, message = "Invalid level range"))]
    level: i32,
}

#[derive(Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct StudentResult {
    pub id: Uuid,
    pub test_id: Uuid,
    pub name: String,
    pub level: i32,
}

async fn register_student(
    Extension(db): Extension<PgPool>,
    Extension(cfg): Extension<Config>,
    Path(test_id): Path<Uuid>,
    Json(req): Json<RegisterStudent>,
) -> Result<impl IntoResponse> {
    req.validate()?;

    let RegisterStudent { name } = req;

    let test = sqlx::query_as!(Test, "SELECT * FROM test WHERE id = $1", test_id)
        .fetch_optional(&db)
        .await?;

    // Does the test exist?
    if let Some(test) = test {
        // Is the test closed?
        if test.closed {
            return Err(Error::Conflict(
                "This test is closed to new registration".to_string(),
            ));
        }

        let existing_result = sqlx::query_as!(
            StudentResult,
            "SELECT * FROM result WHERE (test_id, name) = ($1, $2)",
            test_id,
            name
        )
        .fetch_optional(&db)
        .await?;

        if existing_result.is_some() {
            return Err(Error::Conflict("This name is already taken".to_string()));
        }

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

        let cookie = crate::http::auth::create_cookie("STUDENT_TOKEN", res.id, cfg).await;

        let mut headers = HeaderMap::new();
        headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

        return Ok((StatusCode::ACCEPTED, headers));
    }

    Err(Error::UnprocessableEntity(
        "This test ID is not valid".to_string(),
    ))
}

async fn set_score(
    Extension(db): Extension<PgPool>,
    Extension(student): Extension<StudentResult>,
    Json(req): Json<SetLevel>,
) -> Result<impl IntoResponse> {
    req.validate()?;

    let SetLevel { level } = req;

    sqlx::query_as!(
        StudentResult,
        "UPDATE result SET level = $1 WHERE id = $2",
        level,
        student.id
    )
    .execute(&db)
    .await?;

    let cookie = Cookie::build("TEACHER_TOKEN", "")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok((StatusCode::ACCEPTED, headers))
}
