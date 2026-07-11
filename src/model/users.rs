use serde::{Deserialize, Serialize};

use crate::model::validation::{Password, Username};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct User {
    id:            u64,
    username:      String,
    bio:           String,
    avatar_url:    String,
    password_hash: String,
    is_admin:      bool,
    created_at:    String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username:         Username,
    pub password:         Password,
    pub password_confirm: Password,
    pub invite_code:      String,
}
