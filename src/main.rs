use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;

mod handlers;
use handlers::expense_handlers::{
    create_expense, get_all_expenses, get_expense_by_id, update_expense, delete_expense,
    get_expense_summary,
};
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "sqlite://expenses.db";
    let pool = db::init_db_pool(db_url).await.expect("Failed to connect to database");
    db::setup_database(&pool).await.expect("Failed to set up database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(|| async { "OK" }))
            .route("/expenses", web::post().to(create_expense))
            .route("/expenses", web::get().to(get_all_expenses))
            .route("/expenses/{id}", web::get().to(get_expense_by_id))
            .route("/expenses/{id}", web::put().to(update_expense))
            .route("/expenses/{id}", web::delete().to(delete_expense))
            .route("/expenses/summary", web::get().to(get_expense_summary))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
