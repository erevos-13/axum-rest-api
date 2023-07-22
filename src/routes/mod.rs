use std::sync::Arc;

use crate::{
    lid::AppState,
    structs::{common::DatabaseConfig, user::User},
};
use axum::{body::Body, http::Method, routing::post, Router};
use futures::{StreamExt, TryFutureExt};
use mongodb::{bson::document::ValueAccessError, error::Error, options::ClientOptions, Client};
use tower_http::cors::{Any, CorsLayer};

use self::{
    error_handler::handler_404,
    login_user::{login_user, ResponseUser},
};

pub mod error_handler;
pub mod login_user;

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/user/login", post(login_user))
        .with_state(app_state)
}
