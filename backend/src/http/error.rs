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

    #[error("validation error in request body")]
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

            errors: Option<Vec<String>>,

            auth_error: bool,
        }

        let errors = match &self {
            Self::InvalidEntity(errors) => Some(
                errors
                    .field_errors()
                    .into_iter()
                    .map(|error| {
                        let default = format!("{} is required", error.0);
                        error.1[0]
                            .message
                            .as_ref()
                            .unwrap_or(&std::borrow::Cow::Owned(default))
                            .to_string()
                    })
                    .collect(),
            ),
            _ => None,
        };

        let auth_error = match &self {
            Self::Authorization(_) => true,
            _ => false,
        };

        tracing::error!("API error: {self:?}");

        (
            self.status_code(),
            Json(ErrorResponse {
                message: &self,
                errors,
                auth_error,
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
