use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

use crate::util::{Config, Opt};
use anyhow::Context;
use axum::{
    body::Body,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, Request, StatusCode,
    },
    Extension,
};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use sqlx::PgPool;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

mod auth;
mod error;
mod question;
mod teacher;
mod test;

pub use self::error::Error;
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

pub fn app(opt: Opt, db: PgPool, cfg: Config) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("https://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    Router::new()
        .merge(teacher::router())
        .merge(test::router())
        .merge(question::router())
        .fallback_service(get(|req: Request<Body>| async move {
            let res = ServeDir::new(&opt.static_dir).oneshot(req).await.unwrap(); // serve dir is infallible
            let status = res.status();
            match status {
                StatusCode::NOT_FOUND => {
                    let index_path = PathBuf::from(&opt.static_dir).join("index.html");
                    fs::read_to_string(index_path)
                        .await
                        .map(|index_content| (StatusCode::OK, Html(index_content)).into_response())
                        .unwrap_or_else(|_| {
                            (StatusCode::INTERNAL_SERVER_ERROR, "index.html not found")
                                .into_response()
                        })
                }
                _ => res.into_response(),
            }
        }))
        .layer(Extension(db))
        .layer(Extension(cfg))
        .layer(cors)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

pub async fn serve(opt: Opt, db: PgPool, cfg: Config, tls: RustlsConfig) -> anyhow::Result<()> {
    // let sock_addr = SocketAddr::from((
    //     IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
    //     opt.port,
    // ));

    let listener = tokio::net::TcpListener::bind((opt.addr.as_str(), opt.port))
        .await
        .context("listener failed")?;

    tracing::info!("listening on http://{}:{}", opt.addr, opt.port);

    axum::serve(listener, app(opt, db, cfg).into_make_service())
        .await
        .context("failed to serve api")

    // axum_server::bind_rustls(sock_addr, tls)
    //     .serve(app(opt, db, cfg).into_make_service())
    //     .await
    //     .context("failed to serve api")
}
