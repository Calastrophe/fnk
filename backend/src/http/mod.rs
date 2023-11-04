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
    middleware,
    routing::post,
    Extension,
};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sqlx::PgPool;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

mod auth;
mod error;
mod student;
mod teacher;

pub use self::error::Error;
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

pub fn app(opt: Opt, db: PgPool, cfg: Config) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("https://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    Router::new()
        .fallback_service(get(|req: Request<Body>| async move {
            let res = ServeDir::new(&opt.static_dir).oneshot(req).await.unwrap(); // serve dir is infallible
            let status = res.status();
            match status {
                // If we don't find a file corresponding to the path we serve index.html.
                // If you want to serve a 404 status code instead you can add a route check as shown in
                // https://github.com/rksm/axum-yew-setup/commit/a48abfc8a2947b226cc47cbb3001c8a68a0bb25e
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

                // path was found as a file in the static dir
                _ => res.into_response(),
            }
        }))
        .layer(Extension(db))
        .layer(Extension(cfg))
        .layer(cors)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

pub async fn serve(opt: Opt, db: PgPool, cfg: Config) -> anyhow::Result<()> {
    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    tracing::info!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app(opt, db, cfg).into_make_service())
        .await
        .context("API failed to serve")
}
