use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use mongodb::{bson::doc, options::FindOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::{lid::AppState, structs::user::User};

// use crate::validate_json::ValidatedForm;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(
        length(min = 1, message = "Can not be empty"),
        custom = "validate_unique_username"
    )]
    pub username: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct ResponseUser {
    pub token: String,
}
fn validate_unique_username(username: &str) -> Result<(), ValidationError> {
    dbg!(username);
    if username == "xXxShad0wxXx" {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}
pub async fn login_user(
    State(app_state): State<Arc<AppState>>,
    Json(user): Json<RequestUser>,
    // ValidatedForm(user): ValidatedForm<RequestUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    dbg!(user);
    Ok(Json(ResponseUser {
        token: "token".to_string(),
    }))
}
