use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    State, database::users::{get_user_by_username, insert_user, insert_user_with_invite}, error::{AppError, AuthError}, utility::{
        self,
        validation::{ValidationError, validate_password, validate_username},
    }
};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct User {
    pub id:            i64,
    pub username:      String,
    pub bio:           String,
    pub avatar_url:    String,
    pub password_hash: String,
    pub is_admin:      bool,
    pub created_at:    chrono::DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username:         String,
    pub password:         String,
    pub password_confirm: String,
    pub invite_code:      String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserCredentials {
    pub username: String,
    pub password: String,
    pub new_username: String,
    pub new_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserProfile {
    pub username: String,
    pub password: String,
    pub bio: String,
    pub avatar_url: String,
}

impl User {
    pub async fn create_user(create_user: CreateUser, state: State) -> Result<User, AppError> {
        let username = validate_username(create_user.username.to_owned())?;
        if let Some(_user) = get_user_by_username(create_user.username, &state).await? {
            Err(AuthError::UsernameTaken)?
        }

        if create_user.password != create_user.password_confirm {
            Err(ValidationError::PasswordMismatch)?
        }
        let password = validate_password(create_user.password, &state.config.limits)?;
        let password_hash = utility::hash_password(&password)?;

        let id = state.snowflake.generate().0 as i64;

        let user = match state.config.registration.require_invite {
            true => insert_user_with_invite(id, username, password_hash, create_user.invite_code, &state).await?,
            false => insert_user(id, username, password_hash, &state).await?,
        };

        Ok(user)
    }
}
