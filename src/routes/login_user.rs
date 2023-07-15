use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
}

pub async fn login_user(Json(user): Json<RequestUser>) -> Json<RequestUser> {
    Json(RequestUser {
        username: user.username,
        password: user.password,
    })
}
