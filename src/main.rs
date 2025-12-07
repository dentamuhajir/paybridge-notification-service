use std::env;
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


    // ----------------------------------------------------------
    // Read APP_HOST from environment variables.
    //
    // env::var("APP_HOST") returns:
    //   - Ok(host_string)
    //   - Err(error)
    //
    // unwrap_or_else takes a closure (function) that runs when
    // the env variable does NOT exist.
    //
    // The closure receives one argument: the error.
    // We use "|_|" to ignore the error, since we don't need it.
    //
    // If APP_HOST is missing, return "0.0.0.0".
    // ----------------------------------------------------------
    let host: String = env::var("APP_HOST")
        .unwrap_or_else(|_| "0.0.0.0".into());

    // ----------------------------------------------------------
    // Read APP_PORT from environment variables.
    //
    // Step 1:
    //   env::var("APP_PORT") may fail if not set.
    //   unwrap_or_else(|_| "8081".into()) gives default "8081".
    //
    // Step 2:
    //   parse() converts the string to a number (u16).
    //
    // Step 3:
    //   expect(...) is used to panic if parsing fails.
    //   (Example: APP_PORT="abc" -> invalid number)
    //
    // This is similar to Java:
    //   int port = Integer.parseInt(System.getenv().getOrDefault("APP_PORT", "8081"));
    // ----------------------------------------------------------
    let port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "8081".into())  // default
        .parse()                             // string -> number
        .expect("APP_PORT must be a valid u16 number");

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
