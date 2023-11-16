use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use serde_with::DisplayFromStr;
use validator::ValidationErrors;

/// An API-friendly error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// A SQLx call returned an error.
    ///
    /// The exact error contents are not reported to the user in order to avoid leaking
    /// information about databse internals.
    #[error("an internal database error occurred")]
    Sqlx(#[from] sqlx::Error),

    /// Similarly, we don't want to report random `anyhow` errors to the user.
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("{0}")]
    InvalidEntity(#[from] ValidationErrors),

    #[error("{0}")]
    Authorization(String),

    #[error("{0}")]
    UnprocessableEntity(String),

    #[error("{0}")]
    Conflict(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[serde_with::serde_as]
        #[serde_with::skip_serializing_none]
        #[derive(serde::Serialize)]
        struct ErrorResponse<'a> {
            // Serialize the `Display` output as the error message
            #[serde_as(as = "DisplayFromStr")]
            message: &'a Error,

            errors: Option<&'a ValidationErrors>,
        }

        let errors = match &self {
            Error::InvalidEntity(errors) => Some(errors),
            _ => None,
        };

        tracing::error!("API error: {self:?}");

        (
            self.status_code(),
            Json(ErrorResponse {
                message: &self,
                errors,
            }),
        )
            .into_response()
    }
}

impl Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            Sqlx(_) | Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidEntity(_) | UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Authorization(_) => StatusCode::UNAUTHORIZED,
            Conflict(_) => StatusCode::CONFLICT,
        }
    }
}
