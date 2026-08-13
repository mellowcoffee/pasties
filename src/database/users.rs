use sqlx::FromRow;

use crate::error::{AppError, UserError};
use crate::model::users::User;
use crate::validation::user::{AvatarUrl, Bio, Username};
use crate::State;

pub async fn get_user_by_username(
    username: &Username,
    state: &State,
) -> Result<Option<User>, sqlx::Error> {
    let user: Option<User> = sqlx::query(
        "SELECT (id, username, bio, avatar_url, password_hash, is_admin, created_at)
        FROM users
        WHERE username = $1",
    )
    .bind(username)
    .fetch_optional(&state.pool)
    .await?
    .map(|row| User::from_row(&row))
    .transpose()?;
    Ok(user)
}

pub async fn get_user_by_id(id: i64, state: &State) -> Result<Option<User>, sqlx::Error> {
    let user: Option<User> = sqlx::query(
        "SELECT (id, username, bio, avatar_url, password_hash, is_admin, created_at)
        FROM users
        WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .map(|row| User::from_row(&row))
    .transpose()?;
    Ok(user)
}

pub async fn insert_user(
    id: i64,
    username: Username,
    password_hash: String,
    state: &State,
) -> Result<User, sqlx::Error> {
    let user: User = sqlx::query_as::<_, User>(
        "
        INSERT INTO users (id, username, password_hash)
        VALUES ($1, $2, $3)
    ",
    )
    .bind(id)
    .bind(username)
    .bind(password_hash)
    .fetch_one(&state.pool)
    .await?;
    Ok(user)
}

pub async fn insert_user_with_invite(
    id: i64,
    username: Username,
    password_hash: String,
    invite_code: String,
    state: &State,
) -> Result<User, AppError> {
    let mut tx = state.pool.begin().await?;
    let consumed = sqlx::query(
        "UPDATE invites
        SET (used_by = $1, used_at = now())
        WHERE code = $2 AND used_by IS NULL",
    )
    .bind(id)
    .bind(invite_code)
    .execute(&mut *tx)
    .await?
    .rows_affected();
    if consumed == 0 {
        Err(UserError::InviteUsed)?;
    }

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, username, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, bio, avatar_url, password_hash, is_admin, created_at",
    )
    .bind(id)
    .bind(username)
    .bind(password_hash)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(user)
}

pub async fn update_user_credentials_by_username(
    username: Username,
    new_username: Username,
    new_password_hash: String,
    state: &State,
) -> Result<User, sqlx::Error> {
    let user: User = sqlx::query_as::<_, User>(
        "
        UPDATE users
        SET username = $1, password_hash = $2
        WHERE username = $3
        RETURNING (id, username, bio, avatar_url, password_hash, is_admin, created_at)
    ",
    )
    .bind(new_username)
    .bind(new_password_hash)
    .bind(username)
    .fetch_one(&state.pool)
    .await?;
    Ok(user)
}

pub async fn update_user_profile_by_username(
    username: Username,
    new_bio: Bio,
    new_avatar_url: AvatarUrl,
    state: &State,
) -> Result<User, sqlx::Error> {
    let user: User = sqlx::query_as::<_, User>(
        "
        UPDATE users
        SET bio = $1, avatar_url = $2
        WHERE username = $3
        RETURNING (id, username, bio, avatar_url, password_hash, is_admin, created_at)
    ",
    )
    .bind(new_bio)
    .bind(new_avatar_url)
    .bind(username)
    .fetch_one(&state.pool)
    .await?;
    Ok(user)
}
