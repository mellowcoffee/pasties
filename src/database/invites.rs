use crate::{State, model::invites::Invite};

pub async fn insert_invite(invite: Invite, state: State) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO invites (code, created_by)
        VALUES ($1, $2)",
    )
    .bind(invite.code)
    .bind(invite.created_by as i64)
    .execute(&state.pool)
    .await?;
    Ok(())
}
