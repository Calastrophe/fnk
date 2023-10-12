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
    util::ErrorResponse,
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
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(body.email.to_owned().to_ascii_lowercase())
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {}", e),
                )
            })?;

    if let Some(exists) = user_exists {
        if exists {
            return Err(ErrorResponse::new(
                StatusCode::CONFLICT,
                "User with that email already exists",
            ));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error while hashing password: {}", e),
            )
        })
        .map(|hash| hash.to_string())?;

    let _ = sqlx::query_as!(
        Teacher,
        "INSERT INTO teachers (name,email,password) VALUES ($1, $2, $3) RETURNING *",
        body.name.to_string(),
        body.email.to_string().to_ascii_lowercase(),
        hashed_password
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    let user_response = serde_json::json!({"status": "success"});
    Ok(Json(user_response))
}

pub async fn login(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginTeacherSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let user = sqlx::query_as!(
        Teacher,
        "SELECT * FROM teachers WHERE email = $1",
        body.email.to_ascii_lowercase()
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?
    .ok_or_else(|| ErrorResponse::new(StatusCode::BAD_REQUEST, "Invalid email or password"))?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    };

    if !is_valid {
        return Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "Invalid email or password",
        ));
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

pub async fn logout() -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
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

// TODO: Insert some type of cookie into the test taker's browser for authentication.

// // Fetches all the proctored tests for the logged in teacher
// pub async fn fetch_tests() -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
//     unimplemented!()
// }
//
// // Fetches all the results for the given test for the logged in teacher
// pub async fn fetch_results() -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
//     unimplemented!()
// }
//
// // Updates the current score for the given test
// pub async fn update_score() -> Result<impl IntoResponse, (StatusCode, Json<ErrorResposne>)> {
//     unimplemented!()
// }
//
// // Flags a specific test result for the logged in teacher
// pub async fn flag_result() -> Result<impl IntoResponse, (StatusCode, Json<ErrorResposne>)> {
//     unimplemented!()
// }

pub async fn is_server_up() -> impl IntoResponse {
    "Yes, I am up!"
}
