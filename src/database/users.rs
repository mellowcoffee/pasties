use crate::{State, model::users::User};

pub async fn insert_user(user: User, state: State) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        INSERT INTO users (id, username, password_hash)
        VALUES ($1, $2, $3)
    ",
    )
    .bind(user.id as i64)
    .bind(user.username)
    .bind(user.password_hash)
    .execute(&state.pool)
    .await?;
    Ok(())
}
