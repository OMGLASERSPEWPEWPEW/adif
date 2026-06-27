use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::DatabaseSection;

pub async fn create_pool(config: &DatabaseSection) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.connection_string())
        .await?;
    Ok(pool)
}
