use sqlx::{Pool, Sqlite, SqlitePool};
use sqlx::migrate::Migrator;
use std::path::Path;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Utc};
use uuid::Uuid;

pub type DbPool = SqlitePool;

pub async fn init_db_pool(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePool::connect(db_url).await
}

pub async fn setup_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS expenses (
            id TEXT PRIMARY KEY,
            amount REAL NOT NULL CHECK (amount > 0),
            description TEXT,
            category TEXT NOT NULL DEFAULT 'Others',
            expense_date TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;
    Ok(())
}

