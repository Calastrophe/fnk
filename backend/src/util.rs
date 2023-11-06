use anyhow::{anyhow, Context};
use argon2::{
    password_hash, password_hash::SaltString, Argon2, PasswordHash, PasswordHasher,
    PasswordVerifier,
};
use clap::Parser;
use rand_core::OsRng;
use tokio::task;

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

pub async fn hash(password: String) -> anyhow::Result<String> {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!(e).context("Failed to hash password"))?
            .to_string())
    })
    .await
    .context("panic in hash() fn")?
}

pub async fn verify(password: String, hash: String) -> anyhow::Result<bool> {
    task::spawn_blocking(move || {
        let hash =
            PasswordHash::new(&hash).map_err(|e| anyhow!(e).context("Invalid password hash"))?;

        let res = Argon2::default().verify_password(password.as_bytes(), &hash);

        match res {
            Ok(()) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(e) => Err(anyhow!(e).context("There was a failure when verifying a password")),
        }
    })
    .await
    .context("panic in verify() fn")?
}
