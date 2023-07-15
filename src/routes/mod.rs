use axum::{body::Body, http::Method, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use self::login_user::login_user;

mod login_user;

pub fn create_routes() -> Router<(), Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    Router::new()
        .route("/api/user/login", post(login_user))
        .layer(cors)
}
