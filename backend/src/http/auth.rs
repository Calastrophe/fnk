use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::http::{Error, Result};
use crate::util::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

use axum::{
    http::{header, Request},
    middleware::Next,
    response::IntoResponse,
    Extension, Json,
};

use crate::http::teacher::Teacher;

pub async fn teacher_auth<B>(
    cookie_jar: CookieJar,
    db: Extension<PgPool>,
    cfg: Extension<Config>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse> {
    let token = cookie_jar
        .get("TEACHER_TOKEN")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = token.ok_or_else(|| {
        Error::AuthorizationError("You are not logged in, please provide token".into())
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(cfg.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| Error::AuthorizationError("Invalid token".into()))?
    .claims;

    let teacher_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| Error::AuthorizationError("Invalid token".into()))?;

    let teacher = sqlx::query_as!(
        Teacher,
        "SELECT * FROM teacher WHERE teacher_id = $1",
        teacher_id
    )
    .fetch_optional(&*db)
    .await?;

    let teacher = teacher.ok_or_else(|| {
        Error::AuthorizationError(
            "The teacher belonging to this token no longer exists".to_string(),
        )
    })?;

    req.extensions_mut().insert(teacher);
    Ok(next.run(req).await)
}

pub async fn student_auth<B>(
    cookie_jar: CookieJar,
    db: Extension<PgPool>,
    cfg: Extension<Config>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse> {
    let token = cookie_jar
        .get("STUDENT_TOKEN")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token =
        token.ok_or_else(|| Error::AuthorizationError("You are not logged in".to_string()))?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(cfg.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| Error::AuthorizationError("Invalid token".to_string()))?
    .claims;

    let result_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| Error::AuthorizationError("Invalid token".to_string()))?;

    let result = sqlx::query_as!(
        StudentResult,
        "SELECT * FROM results WHERE id = $1",
        result_id
    )
    .fetch_optional(&*db)
    .await
    .map_err(|e| e.into())?;

    let result = result.ok_or_else(|| {
        Error::AuthorizationError(
            "The student result belonging to this token no longer exists".to_string(),
        )
    })?;

    req.extensions_mut().insert(result);
    Ok(next.run(req).await)
}
