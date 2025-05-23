use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use env_logger;
use log::{info, warn, error, debug};

mod handlers;
use handlers::expense_handlers::create_expense;


mod models;
mod setup;
mod db;

use db::init_db_pool;

// A simple handler function for "/"
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Expense Tracker!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger from environment variable RUST_LOG (default to info level)
    env_logger::init();

    info!("Starting server...");

    // Database connection string for SQLite
    let db_url = "sqlite://expenses.db";

    // Initialize DB poolc
    let pool = init_db_pool(db_url)
        .await
        .expect("Failed to connect to the database");

    // Run migrations on startup
    setup::run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Share DB pool with app data for handlers
            .app_data(web::Data::new(pool.clone()))
            // Basic test route
            .route("/", web::get().to(hello))
            .route("/expenses", web::post().to(create_expense))
            .route("/", web::get().to(|| async { "Server is running!" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
