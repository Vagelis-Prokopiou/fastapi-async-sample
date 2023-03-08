#![allow(clippy::needless_return, non_snake_case)]

pub mod handlers;
pub mod services;
pub mod models;
pub mod utils;

use crate::{
    services::{postgres::PostgresService},
};
use axum::{
    routing::{get, IntoMakeService},
    Router,
};
use hyper::{server::conn::AddrIncoming};

#[derive(Clone)]
pub struct AppState {
    databaseService: PostgresService,
}

pub fn run(
    listener: std::net::TcpListener,
    databaseService: PostgresService,
) -> Result<hyper::server::Server<AddrIncoming, IntoMakeService<Router>>, std::io::Error> {
    let appState = AppState { databaseService };

    let app = axum::routing::Router::new()
        .route("/", get(|| async { "hello from axum server".to_string() }))
        .route("/users", get(handlers::getUsers))
        .with_state(appState);

    let server = axum::Server::from_tcp(listener)
        .expect("axum::Server::from_tcp failed")
        .serve(app.into_make_service());

    return Ok(server);
}
