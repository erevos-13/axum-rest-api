use std::sync::Arc;

use futures::stream::StreamExt;
use mongodb::options::FindOptions;

use crate::{
    db::DB, lid::AppState, my_errors::MyError, responces::UserListResponse, structs::user::User,
};
type Result<T> = std::result::Result<T, MyError>;

pub async fn fetch_users(state: Arc<AppState>, limit: i64, page: i64) -> Result<UserListResponse> {
    let users = vec![];
    let find_options = FindOptions::builder()
        .limit(limit)
        .skip(u64::try_from((page - 1) * limit).unwrap())
        .build();

    let mut cursor = state
        .db
        .user_collection
        .find(None, find_options)
        .await
        .map_err(MyError::MongoQueryError)?;

    let mut json_result: Vec<User> = Vec::new();
    let mut result = String::new();
    while let Some(user) = cursor.current() {
        result.push_str(&format!("{:?}", user.unwrap()));
    }

    Ok(UserListResponse {
        status: "success",
        results: users.len(),
        users,
    })
}
