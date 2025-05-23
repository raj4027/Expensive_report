use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use validator::Validate;

use crate::models::expense::{CreateExpense, Expense};

pub async fn create_expense(
    pool: web::Data<SqlitePool>,
    item: web::Json<CreateExpense>,
) -> impl Responder {
    let new_expense = item.into_inner();

    if let Err(validation_errors) = new_expense.validate() {
        return HttpResponse::BadRequest().json(validation_errors);
    }

    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc().date();

    let result = sqlx::query_as::<_, Expense>(
        "INSERT INTO expenses (id, amount, description, category, expense_date, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)
         RETURNING *",
    )
    .bind(id.to_string())
    .bind(new_expense.amount)
    .bind(new_expense.description.clone())
    .bind(new_expense.category.clone())
    .bind(new_expense.expense_date.to_string())
    .bind(now.to_string())
    .bind(now.to_string())
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(expense) => HttpResponse::Ok().json(expense),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

