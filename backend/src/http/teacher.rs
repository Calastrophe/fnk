use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{routing::post, Extension, Json, Router};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::Rng;
use sqlx::PgPool;
use std::time::Duration;

use super::auth::TokenClaims;
use crate::http::{Error, Result};
use crate::util::Config;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn router() -> Router {
    Router::new().route("/v1/teacher", post(login_teacher).post(register_teacher))
}

#[derive(Deserialize, Validate)]
pub struct RegisterTeacher {
    email: String,
    username: String,
    #[validate(length(min = 8, max = 40))]
    password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginTeacher {
    email: String,
    #[validate(length(min = 8, max = 40))]
    password: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Teacher {
    pub teacher_id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

async fn register_teacher(
    db: Extension<PgPool>,
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
    .execute(&*db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(dbe) if dbe.constraint() == Some("teacher_username_key") => {
            Error::Conflict("This username is already taken.".into())
        }
        _ => e.into(),
    })?;

    Ok(StatusCode::NO_CONTENT)
}

async fn login_teacher(
    db: Extension<PgPool>,
    cfg: Extension<Config>,
    Json(req): Json<LoginTeacher>,
) -> Result<impl IntoResponse> {
    req.validate()?;

    let LoginTeacher { email, password } = req;

    let teacher = sqlx::query_as!(Teacher, "SELECT * FROM teacher WHERE email = $1", email)
        .fetch_optional(&*db)
        .await?;

    if let Some(teacher) = teacher {
        let verified = crate::util::verify(password, teacher.password).await?;

        if verified {
            let now = chrono::Utc::now();
            let iat = now.timestamp() as usize;
            let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
            let claims: TokenClaims = TokenClaims {
                sub: teacher.teacher_id.to_string(),
                exp,
                iat,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(cfg.jwt_secret.as_ref()),
            )
            .unwrap();

            let cookie = Cookie::build("TEACHER_TOKEN", token.to_owned())
                .path("/")
                .max_age(time::Duration::hours(1))
                .same_site(SameSite::Lax)
                .http_only(true)
                .finish();

            let mut headers = HeaderMap::new();
            headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

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
