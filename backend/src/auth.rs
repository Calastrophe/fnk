use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};

use crate::request::{StudentResult, Teacher, TokenClaims};
use crate::{util::ErrorResponse, AppState};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn teacher_auth<B>(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
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
        ErrorResponse::new(
            StatusCode::UNAUTHORIZED,
            "You are not logged in, please provide token",
        )
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| ErrorResponse::new(StatusCode::UNAUTHORIZED, "Invalid token"))?
    .claims;

    let teacher_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| ErrorResponse::new(StatusCode::UNAUTHORIZED, "Invalid token"))?;

    let teacher = sqlx::query_as!(Teacher, "SELECT * FROM teachers WHERE id = $1", teacher_id)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| {
            ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error fetching user from database: {}", e),
            )
        })?;

    let teacher = teacher.ok_or_else(|| {
        ErrorResponse::new(
            StatusCode::UNAUTHORIZED,
            "The teacher belonging to this token no longer exists",
        )
    })?;

    req.extensions_mut().insert(teacher);
    Ok(next.run(req).await)
}

pub async fn student_auth<B>(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
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

    let token = token.ok_or_else(|| {
        ErrorResponse::new(
            StatusCode::UNAUTHORIZED,
            "You are not logged in, please provide token",
        )
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| ErrorResponse::new(StatusCode::UNAUTHORIZED, "Invalid token"))?
    .claims;

    let result_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| ErrorResponse::new(StatusCode::UNAUTHORIZED, "Invalid token"))?;

    let result = sqlx::query_as!(
        StudentResult,
        "SELECT * FROM results WHERE id = $1",
        result_id
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error fetching student result from database: {}", e),
        )
    })?;

    let result = result.ok_or_else(|| {
        ErrorResponse::new(
            StatusCode::UNAUTHORIZED,
            "The student result belonging to this token no longer exists",
        )
    })?;

    req.extensions_mut().insert(result);
    Ok(next.run(req).await)
}
