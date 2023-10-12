use axum::{http::StatusCode, Json};
use clap::Parser;
use serde::Serialize;
use sqlx::{Pool, Postgres};

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
pub struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    pub(crate) log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    pub(crate) addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    pub(crate) port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "../dist")]
    pub(crate) static_dir: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) db_url: String,
    pub(crate) jwt_secret: String,
    pub(crate) jwt_expires_in: String,
    pub(crate) jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        let db_url = std::env::var("DATABASE_URL").expect("The database URL needs to be provided.");
        let jwt_secret = std::env::var("JWT_SECRET").expect("The JWT secret must be specified.");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN")
            .expect("You must specify the amount of time that a JWT will expire in.");
        let jwt_maxage = std::env::var("JWT_MAXAGE")
            .expect("You must specify the max age that the JWT can reach.");
        Config {
            db_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub(crate) db: Pool<Postgres>,
    pub(crate) env: Config,
}

impl AppState {
    pub fn new(db: Pool<Postgres>, env: Config) -> AppState {
        AppState { db, env }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, msg: impl Into<String>) -> (StatusCode, Json<ErrorResponse>) {
        let stat = match status {
            StatusCode::INTERNAL_SERVER_ERROR => "error",
            _ => "fail",
        };

        (
            status,
            Json(ErrorResponse {
                status: stat,
                message: msg.into(),
            }),
        )
    }
}
