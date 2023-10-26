use auth::auth;
use axum::{
    body::Body,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, Request, StatusCode,
    },
    middleware,
    routing::post,
};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use clap::Parser;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use util::*;
mod api;
mod auth;
mod request;
mod response;
mod util;

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    // Allow for our .env file to be read
    dotenv().ok();

    // enable console logging
    tracing_subscriber::fmt::init();
    // Retrieve the configuration file
    let config = Config::init();

    // Create the diesel connections to our database.
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.db_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("Successfully established a connection to the database!");
            pool
        }
        Err(err) => {
            tracing::info!(
                "Failed to establish a connection to the database: {:?}",
                err
            );
            std::process::exit(1);
        }
    };

    // Create the shared state for the server functions
    let state = Arc::new(AppState::new(pool, config));

    // Use CorsLayer to restrict access to backend API
    let cors = CorsLayer::new()
        .allow_origin("https://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .route("/api/hello", get(api::is_server_up))
        .route("/api/auth/register", post(api::register))
        .route("/api/auth/login", post(api::login))
        .route(
            "/api/auth/logout",
            get(api::logout).route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
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
        .with_state(state)
        .layer(cors)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    tracing::info!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}
