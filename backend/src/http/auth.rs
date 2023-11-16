use axum::{
    http::{header, Request},
    middleware::Next,
    response::IntoResponse,
    Extension,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::http::{teacher::Teacher, test::student::StudentResult, Error, Result};
use crate::util::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub async fn teacher_auth<B>(
    cookie_jar: CookieJar,
    Extension(db): Extension<PgPool>,
    Extension(cfg): Extension<Config>,
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
        Error::Authorization("You are not logged in, please provide token".into())
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(cfg.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| Error::Authorization("Invalid token".into()))?
    .claims;

    let teacher_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| Error::Authorization("Invalid token".into()))?;

    let teacher = sqlx::query_as!(
        Teacher,
        "SELECT * FROM teacher WHERE teacher_id = $1",
        teacher_id
    )
    .fetch_optional(&db)
    .await?;

    let teacher = teacher.ok_or_else(|| {
        Error::Authorization("The teacher belonging to this token no longer exists".to_string())
    })?;

    req.extensions_mut().insert(teacher);
    Ok(next.run(req).await)
}

pub async fn student_auth<B>(
    cookie_jar: CookieJar,
    Extension(db): Extension<PgPool>,
    Extension(cfg): Extension<Config>,
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

    let token = token.ok_or_else(|| Error::Authorization("You are not logged in".to_string()))?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(cfg.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| Error::Authorization("Invalid token".to_string()))?
    .claims;

    let student_res_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| Error::Authorization("Invalid token".to_string()))?;

    let result = sqlx::query_as!(
        StudentResult,
        "SELECT * FROM result WHERE id = $1",
        student_res_id
    )
    .fetch_optional(&db)
    .await?;

    let result = result.ok_or_else(|| {
        Error::Authorization(
            "The student result belonging to this token no longer exists".to_string(),
        )
    })?;

    req.extensions_mut().insert(result);
    Ok(next.run(req).await)
}

pub async fn create_cookie(cookie_name: &str, id: uuid::Uuid, cfg: Config) -> String {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build(cookie_name, token.to_owned())
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    cookie.to_string()
}
