use sqlx::sqlite::SqlitePool;

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let sql = std::fs::read_to_string("migrations/init.sql")?;
    sqlx::query(&sql).execute(pool).await?;
    Ok(())
}
