use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::database::users::{get_user_by_username, insert_user, insert_user_with_invite};
use crate::error::{AppError, AuthError};
use crate::utility::{self};
use crate::validation::user::{AvatarUrl, Bio, Password, Username};
use crate::State;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct User {
    pub id:            i64,
    pub username:      Username,
    pub bio:           Bio,
    pub avatar_url:    AvatarUrl,
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
    pub username:     String,
    pub password:     String,
    pub new_username: String,
    pub new_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUserProfile {
    pub username:   String,
    pub password:   String,
    pub bio:        String,
    pub avatar_url: String,
}

impl User {
    pub async fn create_user(create_user: CreateUser, state: State) -> Result<Self, AppError> {
        let username = Username::parse(create_user.username.clone())?;
        if let Some(_user) = get_user_by_username(username.as_inner(), &state).await? {
            Err(AuthError::UsernameTaken)?;
        }

        if create_user.password != create_user.password_confirm {
            Err(AuthError::PasswordMismatch)?;
        }
        let password = Password::parse(create_user.password, &state.config.limits)?;
        let password_hash = utility::hash_password(password.as_inner())?;

        let id = state.snowflake.generate().to_i64();

        let user = match state.config.registration.require_invite {
            true => {
                insert_user_with_invite(
                    id,
                    username.clone().into_inner(),
                    password_hash,
                    create_user.invite_code,
                    &state,
                )
                .await?
            },
            false => insert_user(id, username.into_inner(), password_hash, &state).await?,
        };

        Ok(user)
    }
}
