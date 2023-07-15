use axum::{response::IntoResponse, routing::get, Json, Router};

use crate::routes::create_routes;

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    Json("test")
}

pub async fn run() {
    let app = create_routes();
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
