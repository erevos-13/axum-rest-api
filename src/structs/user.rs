use futures::{Stream, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
#[derive(Serialize, Debug, Clone)]
pub struct User {
    pub _id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: String,
}
pub trait UserModel: DeserializeOwned + Sync + Send + Unpin {}
