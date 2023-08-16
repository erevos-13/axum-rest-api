use serde::Serialize;

use crate::structs::user::User;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub note: User,
}

#[derive(Serialize, Debug)]
pub struct UserListResponse {
    pub status: &'static str,
    pub results: usize,
    pub users: Vec<UserData>,
}
