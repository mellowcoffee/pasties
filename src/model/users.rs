use serde::{Deserialize, Serialize};

use crate::{
    State,
    error::AppError,
    utility::{
        self,
        validation::{validate_password, validate_username},
    },
};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct User {
    pub id:            u64,
    pub username:      String,
    pub bio:           String,
    pub avatar_url:    String,
    pub password_hash: String,
    pub is_admin:      bool,
    pub created_at:    String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username:         String,
    pub password:         String,
    pub password_confirm: String,
    pub invite_code:      String,
}

impl User {
    pub fn new(create_user: CreateUser, state: &State) -> Result<User, AppError> {
        let id = state.snowflake.generate().into();
        let username = validate_username(create_user.username)?;
        let bio = String::new();
        let avatar_url = String::new();
        let password_hash = utility::hash_password(&validate_password(
            create_user.password,
            &state.config.limits,
        )?)?;
        Ok(User {
            id,
            username,
            bio,
            avatar_url,
            password_hash,
            is_admin: false,
            created_at: String::new(),
        })
    }
}
