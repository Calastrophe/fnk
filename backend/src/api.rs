use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde_json::json;
use std::sync::Arc;

use crate::{
    model::{LoginTeacherSchema, RegisterTeacherSchema, Teacher, TokenClaims},
    response::FilteredTeacher,
    AppState,
};

// Modified and retreived from:
// https://codevoweb.com/jwt-authentication-in-rust-using-axum-framework/

/*
 * TODO:
 *      Redesign response schemas for success/failure
 *      Create the associating schemas for tests
 *      Code golf some of the functions here
 */


pub async fn register(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RegisterTeacherSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(body.email.to_owned().to_ascii_lowercase())
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
            })?;

    if let Some(exists) = user_exists {
        if exists {
            return Err(api_err(StatusCode::CONFLICT, "User with that email already exists"));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("Error while hashing password: {}", e))
        })
        .map(|hash| hash.to_string())?;

    let user = sqlx::query_as!(
        Teacher,
        "INSERT INTO teachers (name,email,password) VALUES ($1, $2, $3) RETURNING *",
        body.name.to_string(),
        body.email.to_string().to_ascii_lowercase(),
        hashed_password
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    let user_response = serde_json::json!({"status": "success"});
    Ok(Json(user_response))
}

pub async fn login(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginTeacherSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as!(
        Teacher,
        "SELECT * FROM teachers WHERE email = $1",
        body.email.to_ascii_lowercase()
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        api_err(StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)) 
    })?
    .ok_or_else(|| {
        api_err(StatusCode::BAD_REQUEST, "Invalid email or password")
    })?;

    // TODO: FUNCTIONAL MAGIC EXPECTED HERE

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err(api_err(StatusCode::BAD_REQUEST, "Invalid email or password"));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn logout() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn is_server_up(State(state): State<Arc<AppState>>) -> impl IntoResponse { "Yes, I am up!" }

// Helper function for cleaning up error responses
fn api_err(status: StatusCode, msg: impl ToString) -> (StatusCode, Json<serde_json::Value>) {
    let json_status = match status {
        StatusCode::INTERNAL_SERVER_ERROR => "error",
        _ => "fail",
    };

    (status, Json(serde_json::json!("status": json_status, "message": msg)))
}

