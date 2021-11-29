use chrono::prelude::*;
use serde::{Deserialize, Serialize};

pub struct User {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UserCreateRequest {
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
}

#[derive(Serialize)]
pub struct UserCreateResponce {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
}

impl UserCreateResponce {
    pub fn of(user: User) -> UserCreateResponce {
        UserCreateResponce {
            id: user.id,
            username: user.username,
            firstname: user.firstname,
            lastname: user.lastname,
            email: user.email,
            phone: user.phone,
        }
    }
}

#[derive(Deserialize)]
pub struct UserUpdateRequest {
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
}

#[derive(Serialize)]
pub struct UserUpdateResponse {
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
}

impl UserUpdateResponse {
    pub fn of(user: User) -> UserUpdateResponse {
        UserUpdateResponse {
            id: user.id,
            username: user.username,
            firstname: user.firstname,
            lastname: user.lastname,
            email: user.email,
            phone: user.phone,
        }
    }
}
