use sqlx::PgPool;
use anyhow::{Result, anyhow};

pub async fn check_db(pool: &PgPool) -> Result<()> {
    // Acquire a connection and run a simple query to verify DB is alive
    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await
        .map_err(|e| anyhow!("DB health check failed: {}", e))?;

    if row.0 == 1 {
        Ok(())
    } else {
        Err(anyhow!("Unexpected result from DB health check"))
    }
}