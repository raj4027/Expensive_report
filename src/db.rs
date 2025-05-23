use sqlx::sqlite::SqlitePool;

pub type DbPool = SqlitePool;

pub async fn init_db_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    SqlitePool::connect(database_url).await
}

