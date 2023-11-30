use axum::http::{header, HeaderMap, StatusCode};
use axum::middleware;
use axum::response::IntoResponse;
use axum::{routing::post, Extension, Json, Router};
use axum_extra::extract::cookie::{Cookie, SameSite};
use rand::Rng;
use sqlx::PgPool;
use std::time::Duration;

use crate::http::{Error, Result};
use crate::util::Config;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::auth::teacher_auth;

pub fn router() -> Router {
    Router::new()
        .route("/v1/teacher/login", post(login_teacher))
        .route("/v1/teacher/register", post(register_teacher))
        .route(
            "/v1/teacher/logout",
            post(logout_teacher).route_layer(middleware::from_fn(teacher_auth)),
        )
}

#[derive(Deserialize, Validate)]
pub struct RegisterTeacher {
    #[validate(
        length(min = 1, message = "An email is required"),
        email(message = "The email you entered is invalid")
    )]
    email: String,
    #[validate(length(min = 3, message = "Your username must be atleast 3 characters long"))]
    username: String,
    #[validate(length(
        min = 8,
        max = 40,
        message = "Your password must be between 8 and 40 characters long"
    ))]
    password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginTeacher {
    #[validate(
        length(min = 1, message = "An email is required"),
        email(message = "The email you entered is invalid")
    )]
    email: String,
    #[validate(length(
        min = 8,
        max = 40,
        message = "Your password must be between 8 and 40 characters long"
    ))]
    password: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Teacher {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

async fn register_teacher(
    Extension(db): Extension<PgPool>,
    Json(req): Json<RegisterTeacher>,
) -> Result<StatusCode> {
    req.validate()?;

    let RegisterTeacher {
        email,
        username,
        password,
    } = req;

    let password_hash = crate::util::hash(password).await?;

    sqlx::query!(
        "INSERT INTO teacher (username, email, password) VALUES ($1, $2, $3)",
        username,
        email,
        password_hash
    )
    .execute(&db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(dbe) if dbe.constraint() == Some("teacher_username_key") => {
            Error::Conflict("This username is already taken.".to_string())
        }
        _ => e.into(),
    })?;

    Ok(StatusCode::ACCEPTED)
}

async fn login_teacher(
    Extension(db): Extension<PgPool>,
    Extension(cfg): Extension<Config>,
    Json(req): Json<LoginTeacher>,
) -> Result<impl IntoResponse> {
    req.validate()?;

    let LoginTeacher { email, password } = req;

    let teacher = sqlx::query_as!(Teacher, "SELECT * FROM teacher WHERE email = $1", email)
        .fetch_optional(&db)
        .await?;

    if let Some(teacher) = teacher {
        let verified = crate::util::verify(password, teacher.password).await?;

        if verified {
            let cookie = crate::http::auth::create_cookie("TEACHER_TOKEN", teacher.id, cfg).await;

            let mut headers = HeaderMap::new();
            headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

            return Ok((StatusCode::ACCEPTED, headers));
        }
    }

    // sleep to prevent timing attacks
    let sleep_duration =
        rand::thread_rng().gen_range(Duration::from_millis(100)..=Duration::from_millis(500));
    tokio::time::sleep(sleep_duration).await;

    Err(Error::UnprocessableEntity(
        "Invalid username/password".into(),
    ))
}

async fn logout_teacher() -> Result<impl IntoResponse> {
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
