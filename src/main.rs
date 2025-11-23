use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
use anyhow::Result;

use crate::domain::health::routes::check_db;
use crate::infrastructure::{config::Config, db::create_pool};

// Replace with your crate name if different
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
    let config = Config::load();
    let pool = create_pool(&config).await?;
    let app_state = AppState {
        db_pool: Arc::new(pool),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/health", web::get().to(health_handler))
        // Add other routes here as needed
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}