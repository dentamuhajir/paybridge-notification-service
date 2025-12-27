use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
use anyhow::Result;

use crate::domain::health::routes::check_db;
use crate::infrastructure::{config::Config, db::create_pool};

mod domain;
mod infrastructure;

#[derive(Clone)]
struct AppState {
    db_pool: Arc<sqlx::PgPool>,
}

async fn health_handler(state: web::Data<AppState>) -> impl Responder {
    match check_db(&state.db_pool).await {
        Ok(_) => HttpResponse::Ok().body("Healthy"),
        Err(_) => HttpResponse::ServiceUnavailable().body("DB Unavailable"),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load config and initialize DB
    let config = Config::load();
    let pool = create_pool(&config).await?;
    let app_state = AppState {
        db_pool: Arc::new(pool),
    };

    let host: String = config.app_host;
    let port = config.app_port;

    println!("Starting Paybridge Notification Service...");
    println!("Server running at http://{host}:{port}");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/healthdb", web::get().to(health_handler))
    })
        .bind((host, port))?
        .run();

    // Wait for shutdown
    let result = server.await;

    println!("Server stopped.");

    result?;
    Ok(())
}
