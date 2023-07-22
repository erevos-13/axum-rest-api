use std::sync::Arc;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;

use crate::{db::DB, my_errors::MyError, routes::create_routes};

#[derive(Clone)]
pub struct AppState {
    pub db: DB,
}
pub async fn run() -> Result<(), MyError> {
    let db = DB::init().await?;
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let app = create_routes(Arc::new(AppState { db: db.clone() })).layer(cors);

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
