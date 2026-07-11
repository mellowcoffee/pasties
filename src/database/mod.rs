use anyhow::Result;
use sqlx::PgPool;

pub async fn init_database(pool: &PgPool) -> Result<()> {
    const SCHEMA: &[&str] = &[
        include_str!("sql/create_users.sql"),
        include_str!("sql/create_sessions.sql"),
        include_str!("sql/create_invites.sql"),
        include_str!("sql/create_pages.sql"),
    ];

    for stmt in SCHEMA {
        sqlx::raw_sql(stmt).execute(pool).await?;
    }
    Ok(())
}
