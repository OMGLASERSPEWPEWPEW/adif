use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow)]
pub struct AccountInfo {
    pub id: i32,
    pub name: String,
    pub status: i16,
    pub is_banned: bool,
}

pub async fn find_account_by_id(pool: &PgPool, id: i32) -> anyhow::Result<Option<AccountInfo>> {
    let account = sqlx::query_as::<_, AccountInfo>(
        "SELECT id, name, status, is_banned FROM accounts WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(account)
}

pub async fn find_account_by_name(pool: &PgPool, name: &str) -> anyhow::Result<Option<AccountInfo>> {
    let account = sqlx::query_as::<_, AccountInfo>(
        "SELECT id, name, status, is_banned FROM accounts WHERE name = $1",
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;

    Ok(account)
}

pub async fn find_or_create_account(pool: &PgPool, name: &str) -> anyhow::Result<AccountInfo> {
    if let Some(account) = find_account_by_name(pool, name).await? {
        return Ok(account);
    }

    let account = sqlx::query_as::<_, AccountInfo>(
        "INSERT INTO accounts (name, password_hash, status) \
         VALUES ($1, '', 0) \
         ON CONFLICT (name) DO UPDATE SET login_count = accounts.login_count + 1 \
         RETURNING id, name, status, is_banned",
    )
    .bind(name)
    .fetch_one(pool)
    .await?;

    tracing::info!(account_id = account.id, name = %account.name, "Auto-created account");
    Ok(account)
}
