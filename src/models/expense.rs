use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct Expense {
    pub id: String,               // Changed from Uuid to String to match DB type
    pub amount: f64,
    pub description: Option<String>,
    pub category: String,
    pub expense_date: NaiveDate,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateExpense {
    #[validate(range(min = 0.01))]
    pub amount: f64,

    pub description: Option<String>,

    #[validate(custom = "validate_category")]
    pub category: String,

    #[serde(rename = "date")]
    pub expense_date: NaiveDate,
}

// Allowed categories
const CATEGORIES: [&str; 6] = ["Work", "Personal", "Food", "Transport", "Utilities", "Entertainment"];

fn validate_category(category: &str) -> Result<(), validator::ValidationError> {
    if CATEGORIES.contains(&category) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid category"))
    }
}
