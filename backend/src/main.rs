use anyhow::Context;
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use util::*;
mod http;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }

    dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = Config::init();

    let tls = RustlsConfig::from_pem_file("../ssl/cert.pem", "../ssl/key.pem")
        .await
        .context("Missing certifications")?;

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.db_url)
        .await
        .context("Failed to establish a connection to the database")?;

    sqlx::migrate!().run(&db).await?;

    tracing::info!("Successfully established a connection to the database!");
    http::serve(opt, db, config, tls).await
}
