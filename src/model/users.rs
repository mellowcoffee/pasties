use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::database::users::{
    get_user_by_username, insert_user, insert_user_with_invite,
    update_user_credentials_by_username, update_user_profile_by_username,
};
use crate::error::{AppError, UserError};
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
    pub async fn create_user(
        create_user: CreateUser,
        state: State,
    ) -> Result<Self, AppError> {
        let username = Username::parse(create_user.username)?;
        if let Some(_user) = get_user_by_username(&username, &state).await? {
            Err(UserError::UsernameTaken)?;
        }

        if create_user.password != create_user.password_confirm {
            Err(UserError::PasswordMismatch)?;
        }
        let password = Password::parse(create_user.password, &state.config.limits)?;
        let password_hash = password.hash()?;

        let id = state.snowflake.generate().to_i64();

        let user = match state.config.registration.require_invite {
            true => {
                insert_user_with_invite(
                    id,
                    username.clone(),
                    password_hash,
                    create_user.invite_code,
                    &state,
                )
                .await?
            },
            false => insert_user(id, username, password_hash, &state).await?,
        };

        Ok(user)
    }

    pub async fn update_user_credentials(
        update_user: UpdateUserCredentials,
        state: State,
    ) -> Result<Self, AppError> {
        let username = Username::parse(update_user.username)?;
        let user = match get_user_by_username(&username, &state).await? {
            Some(user) => user,
            None => return Err(UserError::InvalidCredentials)?,
        };

        let password = Password::parse(update_user.password, &state.config.limits)?;
        let new_password =
            Password::parse(update_user.new_password, &state.config.limits)?;

        if !password
            .verify(&user.password_hash)
            .map_err(UserError::from)?
        {
            Err(UserError::InvalidCredentials)?;
        }
        let new_password_hash = new_password.hash()?;

        let new_username = Username::parse(update_user.new_username)?;
        if let Some(_user) = get_user_by_username(&new_username, &state).await? {
            Err(UserError::UsernameTaken)?;
        }

        let new_user = update_user_credentials_by_username(
            username,
            new_username,
            new_password_hash,
            &state,
        )
        .await?;
        Ok(new_user)
    }

    pub async fn update_user_profile(
        update_user: UpdateUserProfile,
        state: State,
    ) -> Result<Self, AppError> {
        let username = Username::parse(update_user.username)?;
        let user = match get_user_by_username(&username, &state).await? {
            Some(user) => user,
            None => Err(UserError::InvalidCredentials)?,
        };

        let password = Password::parse(update_user.password, &state.config.limits)?;

        if !password
            .verify(&user.password_hash)
            .map_err(UserError::from)?
        {
            Err(UserError::InvalidCredentials)?;
        }

        let new_bio = Bio::parse(update_user.bio, &state.config.limits)?;
        let new_avatar_url =
            AvatarUrl::parse(update_user.avatar_url, &state.config.limits)?;

        let new_user = update_user_profile_by_username(
            username,
            new_bio,
            new_avatar_url,
            &state,
        )
        .await?;
        Ok(new_user)
    }
}
