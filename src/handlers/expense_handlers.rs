use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use chrono::{NaiveDate, Utc};
use uuid::Uuid;
use sqlx::Row;
use serde_json::json;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Expense {
    pub id: String,
    pub amount: f64,
    pub description: Option<String>,
    pub category: String,
    pub expense_date: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct ExpenseFilter {
    pub category: Option<String>,
    pub keyword: Option<String>,
}

pub async fn create_expense(
    pool: web::Data<SqlitePool>,
    expense: web::Json<Expense>,
) -> impl Responder {
    if expense.amount <= 0.0 {
        return HttpResponse::BadRequest().json(json!({"error": "Amount must be a positive number"}));
    }
    let valid_categories = vec!["Food", "Transport", "Utilities", "Entertainment", "Others"];
    if !valid_categories.contains(&expense.category.as_str()) {
        return HttpResponse::BadRequest().json(json!({"error": "Invalid category"}));
    }
    if NaiveDate::parse_from_str(&expense.expense_date, "%Y-%m-%d").is_err() {
        return HttpResponse::BadRequest().json(json!({"error": "Invalid date format"}));
    }

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        r#"
        INSERT INTO expenses (id, amount, description, category, expense_date, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(expense.amount)
    .bind(&expense.description)
    .bind(&expense.category)
    .bind(&expense.expense_date)
    .bind(&now)
    .bind(&now)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(json!({"id": id})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Failed to create expense"})),
    }
}

pub async fn get_all_expenses(
    pool: web::Data<SqlitePool>,
    query: web::Query<ExpenseFilter>,
) -> impl Responder {
    let mut sql = String::from("SELECT * FROM expenses WHERE 1=1");
    if let Some(category) = &query.category {
        sql.push_str(" AND category = ?");
    }
    if let Some(keyword) = &query.keyword {
        sql.push_str(" AND description LIKE ?");
    }
    sql.push_str(" ORDER BY expense_date DESC");

    let mut query_builder = sqlx::query_as::<_, Expense>(&sql);
    if let Some(category) = &query.category {
        query_builder = query_builder.bind(category);
    }
    if let Some(keyword) = &query.keyword {
        query_builder = query_builder.bind(format!("%{}%", keyword));
    }

    let expenses = query_builder.fetch_all(pool.get_ref()).await;

    match expenses {
        Ok(expenses) => HttpResponse::Ok().json(expenses), // Now `expenses` is a Vec<Expense>
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch expenses"})),
    }
}

pub async fn get_expense_by_id(
    _pool: web::Data<SqlitePool>,
    _id: web::Path<i32>,
) -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Expense retrieved" }))
}

pub async fn update_expense(
    _pool: web::Data<SqlitePool>,
    _id: web::Path<i32>,
    expense: web::Json<Expense>,
) -> impl Responder {
    HttpResponse::Ok().json(expense.into_inner())
}

pub async fn delete_expense(
    _pool: web::Data<SqlitePool>,
    _id: web::Path<i32>,
) -> impl Responder {
    HttpResponse::NoContent().finish()
}

pub async fn get_expense_summary(
    pool: web::Data<SqlitePool>,
    query: web::Query<ExpenseFilter>,
) -> impl Responder {
    let sql = if query.category.is_some() {
        "SELECT SUM(amount) as total FROM expenses WHERE category = ?"
    } else {
        "SELECT SUM(amount) as total FROM expenses"
    };

    let mut query_builder = sqlx::query(sql);
    if let Some(category) = &query.category {
        query_builder = query_builder.bind(category);
    }

    let result = query_builder.fetch_one(pool.get_ref()).await;

    match result {
        Ok(row) => {
            let total: f64 = row.try_get("total").unwrap_or(0.0);
            HttpResponse::Ok().json(json!({"total": total}))
        }
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Failed to calculate summary"})),
    }
}
