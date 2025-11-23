use sqlx::{PgPool, Pool, Postgres};
use anyhow::Result;

use crate::infrastructure::config::Config;

pub async fn create_pool(config: &Config) -> Result<PgPool> {
    let pool = Pool::<Postgres>::connect(&config.database_url).await?;
    Ok(pool)
}